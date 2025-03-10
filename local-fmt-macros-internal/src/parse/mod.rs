use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

mod alloc;
pub use alloc::*;

mod static_ref;
pub use static_ref::*;

pub enum MessageTokenValue {
    StaticText(String),
    PlaceholderArg(usize),

    // in const, it's expected to be a &'static str
    // in not const, it's expected to be a String
    PlaceholderIdent(Ident),
}

impl MessageTokenValue {
    pub fn to_alloc_token_stream(&self) -> TokenStream {
        match self {
            MessageTokenValue::StaticText(s) => {
                quote::quote! {
                    local_fmt::AllocMessageFormat::AllocText(#s.to_string()),
                }
            }
            MessageTokenValue::PlaceholderArg(n) => {
                let n = *n;
                quote::quote! {
                    local_fmt::AllocMessageFormat::Placeholder(#n),
                }
            }
            MessageTokenValue::PlaceholderIdent(ident) => {
                quote::quote! {
                    local_fmt::AllocMessageFormat::AllocText(#ident),
                }
            }
        }
    }

    pub fn to_static_token_stream(&self) -> TokenStream {
        match self {
            MessageTokenValue::StaticText(s) => {
                let s = s.as_str();
                quote::quote! {
                    local_fmt::RefMessageFormat::RefText(#s),
                }
            }
            MessageTokenValue::PlaceholderArg(n) => {
                let n = *n;
                quote::quote! {
                    local_fmt::RefMessageFormat::Placeholder(#n),
                }
            }
            MessageTokenValue::PlaceholderIdent(ident) => {
                quote::quote! {
                    local_fmt::RefMessageFormat::RefText(#ident),
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MessageValueError {
    #[error("Placeholder number {0} is not found in the message. The hiest number found is {1}")]
    NotFound(usize, usize),
    #[error("not found placeholder value in braces")]
    EmptyPlaceholder,
}

pub trait MessageValue: ToTokens + Sized {
    const MESSAGE_IDENT: &'static str;
    const MESSAGE_ARG_WRAPPER: &'static str;

    fn as_arg(&self) -> Option<usize>;

    fn new_string(s: String) -> Self;

    fn new_placeholder_raw(s: &str) -> Result<Self, MessageValueError>;
}

pub struct MessageToken<V: MessageValue> {
    pub values: Vec<V>,
    pub placeholder_max: Option<usize>,
}

impl<V: MessageValue> MessageToken<V> {
    pub fn args(&self) -> usize {
        self.placeholder_max.map_or(0, |v| v + 1)
    }

    pub fn new(values: Vec<V>) -> Result<Self, MessageValueError> {
        let max = values.iter().filter_map(|v| v.as_arg()).max();

        if let Some(max) = max {
            let mut flag = vec![false; max + 1];
            for v in &values {
                if let Some(n) = v.as_arg() {
                    flag[n] = true;
                }
            }
            for (i, v) in flag.iter().enumerate() {
                if !v {
                    return Err(MessageValueError::NotFound(i, max));
                }
            }
        }

        Ok(Self {
            values,
            placeholder_max: max,
        })
    }
}

impl<V: MessageValue> ToTokens for MessageToken<V> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let count = self.args();
        let values = self
            .values
            .iter()
            .map(|v| v.to_token_stream())
            .collect::<Vec<TokenStream>>();

        let ident = Ident::new(V::MESSAGE_IDENT, proc_macro2::Span::call_site());
        let wrapper = TokenStream::from_str(V::MESSAGE_ARG_WRAPPER).unwrap();

        let token = quote::quote! {
            unsafe { local_fmt::#ident::<#count>::new_unchecked(#wrapper[
                #(
                    #values
                )*
            ]) }
        };

        tokens.extend(token);
    }
}

impl<V: MessageValue> FromStr for MessageToken<V> {
    type Err = MessageValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::<V>::new();

        let mut buffer = Vec::<u8>::new();

        let mut bytes = s.bytes();

        while let Some(byte) = bytes.next() {
            match byte {
                b'{' => {
                    if !buffer.is_empty() {
                        let s = unsafe { String::from_utf8_unchecked(std::mem::take(&mut buffer)) };
                        values.push(V::new_string(s));
                    }

                    let mut placeholder = Vec::new();

                    loop {
                        match bytes.next() {
                            Some(byte) => match byte {
                                b'}' => {
                                    if placeholder.is_empty() {
                                        return Err(MessageValueError::EmptyPlaceholder);
                                    }
                                    let placeholder =
                                        unsafe { std::str::from_utf8_unchecked(&placeholder) };
                                    values.push(V::new_placeholder_raw(placeholder)?);
                                    break;
                                }
                                byte => placeholder.push(byte),
                            },
                            None => {
                                return Err(MessageValueError::EmptyPlaceholder);
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
            let s = unsafe { String::from_utf8_unchecked(buffer) };
            values.push(V::new_string(s));
        }

        Self::new(values)
    }
}
