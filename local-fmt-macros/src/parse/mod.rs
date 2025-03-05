use std::str::FromStr;

use syn::Ident;

pub(crate) enum MessageTokenValue {
    StaticText(String),
    PlaceholderArg(usize),

    // in const, it's expected to be a &'static str
    // in not const, it's expected to be a String
    PlaceholderIdent(Ident),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum MessageTokenValueError {
    #[error("Placeholder number {0} is not found in the message. The hiest number found is {1}")]
    NotFound(usize, usize),
    #[error("Placeholder number {0} is out of range. The hiest number found is {1}")]
    OutOfRange(usize, usize),
    #[error("not found placeholder value in braces")]
    EmptyPlaceholder,
}

pub(crate) struct MessageToken {
    values: Vec<MessageTokenValue>,
}

impl MessageToken {
    pub(crate) fn new(values: Vec<MessageTokenValue>) -> Result<Self, MessageTokenValueError> {
        let max = values
            .iter()
            .filter_map(|v| match v {
                MessageTokenValue::PlaceholderArg(n) => Some(*n),
                _ => None,
            })
            .max();

        if let Some(max) = max {
            let mut flag = vec![false; max + 1];
            for v in &values {
                if let MessageTokenValue::PlaceholderArg(n) = v {
                    flag[*n] = true;
                }
            }
            for (i, v) in flag.iter().enumerate() {
                if !v {
                    return Err(MessageTokenValueError::NotFound(i, max));
                }
            }
        }

        Ok(Self { values })
    }
}

impl FromStr for MessageToken {
    type Err = MessageTokenValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::<MessageTokenValue>::new();

        let mut buffer = Vec::<u8>::new();

        let mut bytes = s.bytes();

        while let Some(byte) = bytes.next() {
            match byte {
                b'{' => {
                    if !buffer.is_empty() {
                        values.push(MessageTokenValue::StaticText(unsafe {
                            String::from_utf8_unchecked(std::mem::take(&mut buffer))
                        }));
                    }

                    let mut placeholder = Vec::new();

                    loop {
                        match bytes.next() {
                            Some(byte) => match byte {
                                b'}' => {
                                    if placeholder.is_empty() {
                                        return Err(MessageTokenValueError::EmptyPlaceholder);
                                    }
                                    let placeholder =
                                        unsafe { std::str::from_utf8_unchecked(&placeholder) };
                                    let number = usize::from_str(placeholder);
                                    match number {
                                        Ok(ok) => {
                                            values.push(MessageTokenValue::PlaceholderArg(ok));
                                        }
                                        Err(_) => {
                                            values.push(MessageTokenValue::PlaceholderIdent(
                                                Ident::new(
                                                    placeholder,
                                                    proc_macro2::Span::call_site(),
                                                ),
                                            ));
                                        }
                                    }
                                    break;
                                }
                                byte => placeholder.push(byte),
                            },
                            None => {
                                return Err(MessageTokenValueError::EmptyPlaceholder);
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
            values.push(MessageTokenValue::StaticText(unsafe {
                String::from_utf8_unchecked(buffer)
            }));
        }

        Self::new(values)
    }
}
