use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::Ident;

pub(crate) enum MessageTokenValue {
    StaticText(String),
    PlaceholderArg(usize),

    // in const, it's expected to be a &'static str
    // in not const, it's expected to be a String
    PlaceholderIdent(Ident),
}

impl MessageTokenValue {
    fn to_token_stream(&self, ident_placeholder: fn(&Ident) -> TokenStream) -> TokenStream {
        match self {
            MessageTokenValue::StaticText(s) => {
                let s = s.as_str();
                quote::quote! {
                    local_fmt::MessageFormat::StaticText(#s),
                }
            }
            MessageTokenValue::PlaceholderArg(n) => {
                let n = *n;
                quote::quote! {
                    local_fmt::MessageFormat::Arg(#n),
                }
            }
            MessageTokenValue::PlaceholderIdent(ident) => ident_placeholder(ident),
        }
    }

    pub(crate) fn to_alloc_token_stream(&self) -> TokenStream {
        self.to_token_stream(|ident| {
            quote::quote! {
                local_fmt::MessageFormat::Text(#ident),
            }
        })
    }

    pub(crate) fn to_static_token_stream(&self) -> TokenStream {
        self.to_token_stream(|ident| {
            quote::quote! {
                local_fmt::MessageFormat::StaticText(#ident),
            }
        })
    }
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
    pub values: Vec<MessageTokenValue>,
    pub placeholder_count: usize,
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

        Ok(Self {
            values,
            placeholder_count: max.unwrap_or(0),
        })
    }

    pub(crate) fn to_vec_token_stream(&self) -> TokenStream {
        let count = self.placeholder_count;
        let values = self
            .values
            .iter()
            .map(|v| v.to_alloc_token_stream())
            .collect::<Vec<TokenStream>>();

        quote::quote! {
            local_fmt::ConstMessage::<#count>::Vec(vec![
                #(
                    #values
                )*
            ]);
        }
    }
    pub(crate) fn to_static_token_stream(&self) -> TokenStream {
        let count = self.placeholder_count;
        let values = self
            .values
            .iter()
            .map(|v| v.to_static_token_stream())
            .collect::<Vec<TokenStream>>();

        quote::quote! {
            local_fmt::ConstMessage::<#count>::Static(&[
                #(
                    #values
                )*
            ]);
        }
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
