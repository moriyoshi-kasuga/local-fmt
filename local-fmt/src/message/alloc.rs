use std::{fmt::Display, str::FromStr};

use super::CreateMessageError;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllocMessageFormat {
    AllocText(String),
    Placeholder(usize),
}

impl Display for AllocMessageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocMessageFormat::AllocText(text) => write!(f, "{}", text),
            AllocMessageFormat::Placeholder(n) => write!(f, "{{{}}}", n),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllocMessage<const N: usize> {
    format: Vec<AllocMessageFormat>,
}

impl<const N: usize> AllocMessage<N> {
    /// # Safety
    /// The caller must ensure that the format is correct.
    pub unsafe fn new_unchecked(format: Vec<AllocMessageFormat>) -> Self {
        Self { format }
    }

    pub fn new(format: Vec<AllocMessageFormat>) -> Result<Self, CreateMessageError> {
        let mut numbers = Vec::new();

        let mut current = 0;

        while format.len() > current {
            if let AllocMessageFormat::Placeholder(n) = format[current] {
                if n >= numbers.len() {
                    numbers.resize_with(n + 1, Default::default);
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

        Ok(Self { format })
    }

    pub fn new_panic(format: Vec<AllocMessageFormat>) -> Self {
        match Self::new(format) {
            Ok(message) => message,
            Err(error) => error.panic(),
        }
    }

    pub fn format(&self, args: &[&str; N]) -> String {
        let mut result = String::new();

        for format in &self.format {
            match format {
                AllocMessageFormat::AllocText(text) => result.push_str(text),
                AllocMessageFormat::Placeholder(n) => result.push_str(args[*n]),
            }
        }

        result
    }

    pub fn len(&self) -> usize {
        self.format.len()
    }

    pub fn is_empty(&self) -> bool {
        self.format.is_empty()
    }

    pub fn formats(&self) -> &Vec<AllocMessageFormat> {
        &self.format
    }
}

impl<const N: usize> Display for AllocMessage<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for format in &self.format {
            write!(f, "{}", format)?;
        }
        Ok(())
    }
}

impl<const N: usize> FromStr for AllocMessage<N> {
    type Err = CreateMessageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut formats = Vec::<AllocMessageFormat>::new();

        let mut buffer = Vec::<u8>::new();

        let mut bytes = s.bytes();

        while let Some(byte) = bytes.next() {
            match byte {
                b'{' => {
                    if !buffer.is_empty() {
                        formats.push(AllocMessageFormat::AllocText(unsafe {
                            String::from_utf8_unchecked(std::mem::take(&mut buffer))
                        }));
                    }

                    let mut number = None::<usize>;

                    loop {
                        match bytes.next() {
                            Some(byte) => match byte {
                                b'}' => {
                                    if let Some(number) = number {
                                        formats.push(AllocMessageFormat::Placeholder(number));
                                        break;
                                    } else {
                                        return Err(CreateMessageError::EmptyPlaceholder);
                                    }
                                }
                                b'0'..=b'9' => {
                                    let mut num = number.unwrap_or(0);
                                    num *= 10;
                                    num += (byte - b'0') as usize;
                                    number = Some(num);
                                }
                                _ => match number {
                                    Some(number) => {
                                        return Err(CreateMessageError::InvalidNumber {
                                            number,
                                            n: N,
                                        });
                                    }
                                    None => {
                                        return Err(CreateMessageError::EmptyPlaceholder);
                                    }
                                },
                            },
                            None => {
                                return Err(CreateMessageError::EmptyPlaceholder);
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
            formats.push(AllocMessageFormat::AllocText(unsafe {
                String::from_utf8_unchecked(buffer)
            }));
        }

        Self::new(formats)
    }
}
