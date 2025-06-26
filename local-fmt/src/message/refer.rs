use std::fmt::Display;

use crate::{const_i128_to_str, const_u128_to_str, UtilBufWrapper};

use super::CreateMessageError;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RefMessageFormat<'a> {
    RefText(&'a str),
    UNumber(u128),
    INumber(i128),
    Placeholder(usize),
}

impl Display for RefMessageFormat<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefMessageFormat::RefText(text) => write!(f, "{}", text),
            RefMessageFormat::UNumber(n) => write!(f, "{}", n),
            RefMessageFormat::INumber(n) => write!(f, "{}", n),
            RefMessageFormat::Placeholder(n) => write!(f, "{{{}}}", n),
        }
    }
}

pub type StaticMessage<const N: usize> = RefMessage<'static, N>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RefMessage<'a, const N: usize> {
    formats: &'a [RefMessageFormat<'a>],
}

impl<'a, const N: usize> RefMessage<'a, N> {
    /// # Safety
    /// The caller must ensure that the `formats` slice is valid.
    pub const unsafe fn new_unchecked(formats: &'a [RefMessageFormat<'a>]) -> Self {
        Self { formats }
    }

    pub const fn new(formats: &'a [RefMessageFormat<'a>]) -> Result<Self, CreateMessageError> {
        let mut numbers = [false; N];

        let mut current = 0;

        while formats.len() > current {
            if let RefMessageFormat::Placeholder(n) = formats[current] {
                if n >= N {
                    return Err(CreateMessageError::InvalidNumber { number: n, n: N });
                }
                numbers[n] = true;
            }
            current += 1;
        }

        let mut current = 0;

        while numbers.len() > current {
            if !numbers[current] {
                return Err(CreateMessageError::WithoutNumber {
                    number: current,
                    n: N,
                });
            }
            current += 1;
        }

        Ok(Self { formats })
    }

    #[track_caller]
    pub const fn new_panic(formats: &'a [RefMessageFormat<'a>]) -> Self {
        match Self::new(formats) {
            Ok(message) => message,
            Err(error) => error.panic(),
        }
    }

    pub fn format(&self, args: &[&str; N]) -> String {
        let mut result = String::new();

        for format in self.formats {
            match format {
                RefMessageFormat::RefText(text) => result.push_str(text),
                RefMessageFormat::UNumber(n) => result.push_str(&n.to_string()),
                RefMessageFormat::INumber(n) => result.push_str(&n.to_string()),
                RefMessageFormat::Placeholder(n) => result.push_str(args[*n]),
            }
        }

        result
    }

    /// Returns the number of format elements in this message.
    /// 
    /// This counts both text segments and placeholders.
    /// 
    /// # Example
    /// ```
    /// use local_fmt::{RefMessageFormat, RefMessage};
    /// 
    /// const MSG: RefMessage<'static, 1> = RefMessage::new_panic(&[
    ///     RefMessageFormat::RefText("Hello "),
    ///     RefMessageFormat::Placeholder(0),
    ///     RefMessageFormat::RefText("!"),
    /// ]);
    /// assert_eq!(MSG.len(), 3);
    /// ```
    pub const fn len(&self) -> usize {
        self.formats.len()
    }

    /// Returns true if this message has no format elements.
    /// 
    /// # Example
    /// ```
    /// use local_fmt::RefMessage;
    /// 
    /// const MSG: RefMessage<'static, 0> = RefMessage::new_panic(&[]);
    /// assert!(MSG.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.formats.is_empty()
    }

    /// Returns a reference to the internal format elements.
    /// 
    /// This provides access to the underlying representation of the message format,
    /// which consists of text segments, placeholders, and numeric constants.
    /// 
    /// # Example
    /// ```
    /// use local_fmt::{RefMessageFormat, RefMessage};
    /// 
    /// const MSG: RefMessage<'static, 1> = RefMessage::new_panic(&[
    ///     RefMessageFormat::RefText("Count: "),
    ///     RefMessageFormat::Placeholder(0),
    /// ]);
    /// assert_eq!(MSG.formats().len(), 2);
    /// ```
    pub const fn formats(&self) -> &'a [RefMessageFormat<'a>] {
        self.formats
    }
}

impl<const N: usize> StaticMessage<N> {
    /// Formats the message with the given arguments.
    ///
    /// # Safety
    /// The SIZE parameter must be large enough to hold the entire formatted string.
    /// If the buffer is too small, this function will panic with an array bounds error.
    /// To estimate required size:
    /// - Calculate the total length of all text segments
    /// - Add the total length of all argument strings
    /// - Add some extra space for safety margin
    ///
    /// Ensure that the total of all characters does not exceed SIZE
    ///
    /// # Example
    /// ```
    /// use local_fmt::{RefMessageFormat, StaticMessage, utils::UtilBufWrapper};
    ///
    /// const MESSAGE: StaticMessage<2> = StaticMessage::new_panic(&[
    ///    RefMessageFormat::RefText("Hello! "),
    ///    RefMessageFormat::Placeholder(0),
    ///    RefMessageFormat::RefText(" World! "),
    ///    RefMessageFormat::Placeholder(1),
    ///    RefMessageFormat::RefText("!"),
    /// ]);
    ///
    /// const TEXT: &str = {
    ///     const BUF: UtilBufWrapper<32> = unsafe {
    ///         MESSAGE.const_format::<32>(&[b"Beautiful", b"Rust!"])
    ///     };
    ///     BUF.as_str()
    /// };
    ///
    /// assert_eq!(TEXT, "Hello! Beautiful World! Rust!!");
    /// ```
    pub const unsafe fn const_format<const SIZE: usize>(
        &self,
        args: &[&[u8]; N],
    ) -> UtilBufWrapper<SIZE> {
        let mut buf = [0u8; SIZE];
        let mut total = 0;

        let mut i = 0;
        while i < self.formats.len() {
            macro_rules! process {
                ($bytes:expr) => {
                    match $bytes {
                        bytes => {
                            let len = bytes.len();
                            // Check if we have enough buffer space
                            if total + len > SIZE {
                                panic!("Buffer too small for const_format. Increase SIZE parameter or reduce message length.");
                            }
                            let mut j = 0;
                            while j < len {
                                buf[total] = bytes[j];
                                total += 1;
                                j += 1;
                            }
                            i += 1;
                        }
                    }
                };
            }
            match &self.formats[i] {
                RefMessageFormat::RefText(text) => process!(text.as_bytes()),
                RefMessageFormat::UNumber(n) => process!(const_u128_to_str(*n).buffer()),
                RefMessageFormat::INumber(n) => process!(const_i128_to_str(*n).buffer()),
                RefMessageFormat::Placeholder(n) => process!(args[*n]),
            }
        }

        UtilBufWrapper::new(buf, total)
    }
}

impl<const N: usize> Display for RefMessage<'_, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for format in self.formats {
            write!(f, "{}", format)?;
        }

        Ok(())
    }
}
