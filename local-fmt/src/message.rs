use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageFormat {
    Text(String),
    StaticText(&'static str),
    Arg(usize),
}

/// N is argument length
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstMessage<const N: usize> {
    Vec(Vec<MessageFormat>),
    Static(&'static [MessageFormat]),
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ConstMessageError {
    #[error("Invalid argument number: {number} is out of the allowed range (0 <= number < {n}).")]
    InvalidNumber { number: usize, n: usize },
    #[error("Missing argument number: {number} is not found within the allowed range (0 <= number < {n}).")]
    WithoutNumber { number: usize, n: usize },
}

impl<const N: usize> ConstMessage<N> {
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

        Ok(Self::Vec(formats))
    }

    pub const fn args_len(&self) -> usize {
        N
    }

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
    fn as_ref(&self) -> &[MessageFormat] {
        match self {
            Self::Vec(v) => v.as_ref(),
            Self::Static(v) => v,
        }
    }
}

impl<const N: usize> Display for ConstMessage<N> {
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
