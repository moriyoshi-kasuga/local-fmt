use std::{fmt::Display, str::FromStr};

/// Represents different formats a message can take.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageFormat {
    Text(String),
    StaticText(&'static str),
    Arg(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstMessage<const N: usize> {
    Vec(Vec<MessageFormat>),
    Static(&'static [MessageFormat]),
}

/// Errors that can occur when working with constant messages.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ConstMessageError {
    #[error("Invalid argument number: {number} is out of the allowed range (0 <= number < {n}).")]
    InvalidNumber { number: usize, n: usize },
    #[error("Missing argument number: {number} is not found within the allowed range (0 <= number < {n}).")]
    WithoutNumber { number: usize, n: usize },
}

impl<const N: usize> ConstMessage<N> {
    /// Creates a new static constant message, checking for argument validity.
    ///
    /// # Panics
    ///
    /// Panics if the argument numbers are invalid or missing.
    ///
    /// # Examples
    ///
    /// ```
    /// use local_fmt::message::{ConstMessage, MessageFormat};
    ///
    /// const FORMATS: &[MessageFormat] = &[MessageFormat::Arg(0), MessageFormat::StaticText(" world!")];
    /// let message = ConstMessage::<1>::new_static(FORMATS);
    ///
    /// assert_eq!(message.format(&["Hello"]), "Hello world!");
    /// ```
    pub const fn new_static(formats: &'static [MessageFormat]) -> Self {
        let formats = match Self::const_check(formats) {
            Ok(ok) => ok,
            Err(err) => {
                match err {
                    ConstMessageError::InvalidNumber { .. } => {
                        panic!("Invalid argument number: the provided number is out of the allowed range.")
                    }
                    ConstMessageError::WithoutNumber { .. } => {
                        panic!("Argument number missing: not all required argument numbers are present.")
                    }
                }
            }
        };

        Self::Static(formats)
    }

    /// Checks the validity of the argument numbers in the message formats.
    ///
    /// Returns an error if any argument number is invalid or missing.
    ///
    /// # Examples
    ///
    /// ```
    /// use local_fmt::message::{ConstMessage, MessageFormat, ConstMessageError};
    ///
    /// const FORMATS: &[MessageFormat] = &[MessageFormat::Arg(0)];
    /// let result = ConstMessage::<1>::const_check(FORMATS);
    /// assert!(result.is_ok());
    ///
    /// const MISS_FORMATS: &[MessageFormat] = &[MessageFormat::Arg(1)];
    /// let result = ConstMessage::<2>::const_check(MISS_FORMATS);
    /// assert_eq!(result, Err(ConstMessageError::WithoutNumber { number: 0, n: 2 }));
    /// ```
    pub const fn const_check(
        formats: &[MessageFormat],
    ) -> Result<&[MessageFormat], ConstMessageError> {
        let mut numbers = [false; N];

        let mut current = 0;

        while formats.len() > current {
            if let MessageFormat::Arg(n) = formats[current] {
                if n >= N {
                    return Err(ConstMessageError::InvalidNumber { number: n, n: N });
                }
                numbers[n] = true;
            }
            current += 1;
        }

        let mut current = 0;

        while numbers.len() > current {
            if !numbers[current] {
                return Err(ConstMessageError::WithoutNumber {
                    number: current,
                    n: N,
                });
            }
            current += 1;
        }

        Ok(formats)
    }

    /// Creates a new constant message from a vector of message formats.
    ///
    /// Returns an error if the argument numbers are invalid or missing.
    ///
    /// # Examples
    ///
    /// ```
    /// use local_fmt::message::{ConstMessage, MessageFormat};
    ///
    /// let formats = vec![MessageFormat::Arg(0), MessageFormat::StaticText(" world!")];
    /// let message = ConstMessage::<1>::new(formats).unwrap();
    ///
    /// assert_eq!(message.format(&["Hello"]), "Hello world!");
    /// ```
    pub fn new(formats: Vec<MessageFormat>) -> Result<Self, ConstMessageError> {
        Self::const_check(&formats)?;

        Ok(Self::Vec(formats))
    }

    /// Returns the number of arguments expected by the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use local_fmt::message::ConstMessage;
    ///
    /// let message = ConstMessage::<2>::Vec(vec![]);
    /// assert_eq!(message.args_len(), 2);
    /// ```
    pub const fn args_len(&self) -> usize {
        N
    }

    /// Formats the message using the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A slice of string references to be used as arguments in the message.
    ///
    /// # Returns
    ///
    /// A formatted string with the arguments inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use local_fmt::message::{ConstMessage, MessageFormat};
    ///
    /// let formats = vec![MessageFormat::Arg(0), MessageFormat::StaticText(" world!")];
    /// let message = ConstMessage::<1>::new(formats).unwrap();
    /// let formatted = message.format(&["Hello"]);
    /// assert_eq!(formatted, "Hello world!");
    /// ```
    pub fn format(&self, args: &[&str; N]) -> String {
        let mut text = String::new();

        for i in self.as_ref() {
            match i {
                MessageFormat::Text(s) => text.push_str(s),
                MessageFormat::Arg(n) => text.push_str(args[*n]),
                MessageFormat::StaticText(s) => text.push_str(s),
            }
        }

        text
    }
}

impl<const N: usize> AsRef<[MessageFormat]> for ConstMessage<N> {
    /// Returns a reference to the underlying message formats.
    fn as_ref(&self) -> &[MessageFormat] {
        match self {
            Self::Vec(v) => v.as_ref(),
            Self::Static(v) => v,
        }
    }
}

impl<const N: usize> Display for ConstMessage<N> {
    /// Formats the message for display.
    ///
    /// # Examples
    ///
    /// ```
    /// use local_fmt::message::{ConstMessage, MessageFormat};
    ///
    /// let formats = vec![MessageFormat::Arg(0), MessageFormat::StaticText(" world!")];
    /// let message = ConstMessage::<1>::new(formats).unwrap();
    /// assert_eq!(message.to_string(), "{0} world!");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.as_ref() {
            match i {
                MessageFormat::Text(s) => write!(f, "{}", s)?,
                MessageFormat::Arg(n) => write!(f, "{{{}}}", n)?,
                MessageFormat::StaticText(s) => write!(f, "{}", s)?,
            }
        }

        Ok(())
    }
}

impl<const N: usize> FromStr for ConstMessage<N> {
    type Err = ConstMessageError;

    /// Parses a string into a constant message, extracting message formats.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse into a constant message.
    ///         The string can contain arguments in the form of `{n}` where `n` is the argument number.
    ///         To escape the `{` character, use `\\{`.
    ///
    /// # Returns
    ///
    /// A result containing the constant message or an error if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use local_fmt::message::{ConstMessage, MessageFormat};
    /// use std::str::FromStr;
    ///
    /// let message = ConstMessage::<1>::from_str("{0} world!").unwrap();
    /// let vec = vec![MessageFormat::Arg(0), MessageFormat::Text(" world!".to_string())];
    /// assert_eq!(message, ConstMessage::<1>::new(vec).unwrap());
    ///
    /// // Escaping the `{` character
    /// let message = ConstMessage::<1>::from_str("{0} \\{0} world!").unwrap();
    /// let vec = vec![MessageFormat::Arg(0), MessageFormat::Text(" {0} world!".to_string())];
    /// assert_eq!(message, ConstMessage::<1>::new(vec).unwrap());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut formats = Vec::<MessageFormat>::new();

        let mut buffer = Vec::<u8>::new();

        let mut bytes = s.bytes();

        while let Some(byte) = bytes.next() {
            match byte {
                b'{' => {
                    if !buffer.is_empty() {
                        formats.push(MessageFormat::Text(unsafe {
                            String::from_utf8_unchecked(std::mem::take(&mut buffer))
                        }));
                    }

                    let mut number = 0;

                    loop {
                        match bytes.next() {
                            Some(byte) => match byte {
                                b'}' => {
                                    formats.push(MessageFormat::Arg(number));
                                    break;
                                }
                                b'0'..=b'9' => {
                                    number *= 10;
                                    number += (byte - b'0') as usize;
                                }
                                _ => {
                                    return Err(ConstMessageError::InvalidNumber { number, n: N });
                                }
                            },
                            None => {
                                return Err(ConstMessageError::WithoutNumber { number, n: N });
                            }
                        }
                    }
                }
                b'\\' => {
                    if let Some(byte) = bytes.next() {
                        match byte {
                            b'{' => buffer.push(b'{'),
                            _ => {
                                buffer.push(b'\\');
                                buffer.push(byte);
                            }
                        }
                    } else {
                        buffer.push(b'\\');
                    }
                }
                _ => buffer.push(byte),
            }
        }

        if !buffer.is_empty() {
            formats.push(MessageFormat::Text(unsafe {
                String::from_utf8_unchecked(buffer)
            }));
        }

        Self::new(formats)
    }
}
