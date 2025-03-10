use std::num::IntErrorKind;

use quote::ToTokens;
use syn::Ident;

use super::{MessageToken, MessageValue};

pub type StaticMessage = MessageToken<StaticMessageValue>;

pub enum StaticMessageValue {
    StaticText(String),
    UNumberIdent(Ident),
    INumberIdent(Ident),
    Placeholder(usize),
    StaticTextIdent(Ident),
}

impl MessageValue for StaticMessageValue {
    const MESSAGE_IDENT: &'static str = "StaticMessage";
    const MESSAGE_ARG_WRAPPER: &'static str = "&";

    fn as_arg(&self) -> Option<usize> {
        match self {
            StaticMessageValue::Placeholder(n) => Some(*n),
            _ => None,
        }
    }

    fn new_string(s: String) -> Self {
        Self::StaticText(s)
    }

    fn new_placeholder_raw(s: &str) -> Result<Self, super::MessageValueError> {
        if let Some(ident) = s.strip_prefix("u:") {
            Ok(Self::UNumberIdent(Ident::new(
                ident,
                proc_macro2::Span::call_site(),
            )))
        } else if let Some(ident) = s.strip_prefix("i:") {
            Ok(Self::INumberIdent(Ident::new(
                ident,
                proc_macro2::Span::call_site(),
            )))
        } else {
            let number = s.parse::<usize>();
            match number {
                Ok(ok) => Ok(Self::Placeholder(ok)),
                Err(err) if IntErrorKind::InvalidDigit == *err.kind() => Ok(Self::StaticTextIdent(
                    Ident::new(s, proc_macro2::Span::call_site()),
                )),
                Err(err) => {
                    panic!("Invalid placeholder on {{{}}}: {}", s, err);
                }
            }
        }
    }
}

impl ToTokens for StaticMessageValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            StaticMessageValue::StaticText(s) => {
                tokens.extend(quote::quote! {
                    local_fmt::RefMessageFormat::RefText(#s),
                });
            }
            StaticMessageValue::UNumberIdent(ident) => {
                tokens.extend(quote::quote! {
                    local_fmt::RefMessageFormat::UNumber(#ident),
                });
            }
            StaticMessageValue::INumberIdent(ident) => {
                tokens.extend(quote::quote! {
                    local_fmt::RefMessageFormat::INumber(#ident),
                });
            }
            StaticMessageValue::Placeholder(n) => {
                let n = *n;
                tokens.extend(quote::quote! {
                    local_fmt::RefMessageFormat::Placeholder(#n),
                });
            }
            StaticMessageValue::StaticTextIdent(ident) => {
                tokens.extend(quote::quote! {
                    local_fmt::RefMessageFormat::RefText(#ident),
                });
            }
        }
    }
}
