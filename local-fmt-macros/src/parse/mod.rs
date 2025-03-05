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
#[error("not found placeholder value in braces")]
pub(crate) struct MessageTokenEmptyPlaceholderError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum MessageTokenValueError {
    #[error("Placeholder number {0} is not found in the message. The hiest number found is {1}")]
    NotFound(usize, usize),
    #[error("{0}")]
    EmptyPlaceholder(#[from] MessageTokenEmptyPlaceholderError),
}

pub(crate) struct MessageToken {
    pub values: Vec<MessageTokenValue>,
    pub placeholder_max: Option<usize>,
}

impl MessageToken {
    pub(crate) fn check(&self) -> Result<(), MessageTokenValueError> {
        if let Some(max) = self.placeholder_max {
            let mut flag = vec![false; max + 1];
            for v in &self.values {
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

        Ok(())
    }

    pub(crate) fn new_unchecked(values: Vec<MessageTokenValue>) -> Self {
        let max = values
            .iter()
            .filter_map(|v| match v {
                MessageTokenValue::PlaceholderArg(n) => Some(*n),
                _ => None,
            })
            .max();

        Self {
            values,
            placeholder_max: max,
        }
    }

    pub(crate) fn to_vec_token_stream(&self) -> TokenStream {
        let count = self.placeholder_max.map_or(0, |v| v + 1);
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
            ])
        }
    }

    pub(crate) fn to_static_token_stream(&self) -> TokenStream {
        let count = self.placeholder_max.map_or(0, |v| v + 1);
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
            ])
        }
    }
    pub(crate) fn from_str_unchecked(s: &str) -> Result<Self, MessageTokenEmptyPlaceholderError> {
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
                                        return Err(MessageTokenEmptyPlaceholderError);
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
                                return Err(MessageTokenEmptyPlaceholderError);
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

        Ok(Self::new_unchecked(values))
    }
}

impl FromStr for MessageToken {
    type Err = MessageTokenValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = Self::from_str_unchecked(s)?;

        result.check()?;

        Ok(result)
    }
}
