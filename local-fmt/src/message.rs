use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageFormat {
    Text(String),
    StaticText(&'static str),
    Arg(usize),
}

/// N is argument length
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstMessage<const N: usize>(Vec<MessageFormat>);

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ConstMessageError {
    #[error("invalid number: {number} (please 0 <= number < {n})")]
    InvalidNumber { number: usize, n: usize },
    #[error("without number: {number} (not found in 0 <= number < {n})")]
    WithoutNumber { number: usize, n: usize },
}

impl<const N: usize> ConstMessage<N> {
    pub const fn const_check_and_panic(formats: &[MessageFormat]) -> &[MessageFormat] {
        match Self::const_check(formats) {
            Ok(ok) => ok,
            Err(err) => match err {
                ConstMessageError::InvalidNumber { .. } => {
                    panic!("has invalid number arg")
                }
                ConstMessageError::WithoutNumber { .. } => {
                    panic!("has without number arg")
                }
            },
        }
    }

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

    pub fn new(formats: Vec<MessageFormat>) -> Result<Self, ConstMessageError> {
        Self::const_check(&formats)?;

        Ok(Self(formats))
    }

    /// # Safety
    /// fill arg number by `0 <= n < N`
    pub const unsafe fn new_unchecked(formats: Vec<MessageFormat>) -> Self {
        Self(formats)
    }

    pub const fn args_len(&self) -> usize {
        N
    }

    pub fn format(&self, args: &[&str; N]) -> String {
        let mut text = String::new();

        for i in &self.0 {
            match i {
                MessageFormat::Text(s) => text.push_str(s),
                MessageFormat::Arg(n) => text.push_str(args[*n]),
                MessageFormat::StaticText(s) => text.push_str(s),
            }
        }

        text
    }
}

impl<const N: usize> Display for ConstMessage<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut formats = Vec::<MessageFormat>::new();

        let mut buffer = Vec::<u8>::new();

        let mut bytes = s.bytes();

        while let Some(byte) = bytes.next() {
            match byte {
                b'{' => {
                    formats.push(MessageFormat::Text(unsafe {
                        String::from_utf8_unchecked(std::mem::take(&mut buffer))
                    }));

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
