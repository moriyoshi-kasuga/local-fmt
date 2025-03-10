use std::num::IntErrorKind;

use quote::ToTokens;
use syn::Ident;

use super::{MessageToken, MessageValue};

pub type AllocMessage = MessageToken<AllocMessageValue>;

pub enum AllocMessageValue {
    AllocText(String),
    Placeholder(usize),
    AllocTextIdent(Ident),
}

impl MessageValue for AllocMessageValue {
    const MESSAGE_IDENT: &'static str = "AllocMessage";
    const MESSAGE_ARG_WRAPPER: &'static str = "vec!";

    fn as_arg(&self) -> Option<usize> {
        match self {
            AllocMessageValue::Placeholder(n) => Some(*n),
            _ => None,
        }
    }

    fn new_string(s: String) -> Self {
        Self::AllocText(s)
    }

    fn new_placeholder_raw(s: &str) -> Result<Self, super::MessageValueError> {
        let number = s.parse::<usize>();
        match number {
            Ok(ok) => Ok(Self::Placeholder(ok)),
            Err(err) if IntErrorKind::InvalidDigit == *err.kind() => Ok(Self::AllocTextIdent(
                Ident::new(s, proc_macro2::Span::call_site()),
            )),
            Err(err) => {
                panic!("Invalid placeholder on {{{}}}: {}", s, err);
            }
        }
    }
}

impl ToTokens for AllocMessageValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AllocMessageValue::AllocText(s) => {
                tokens.extend(quote::quote! {
                    local_fmt::AllocMessageFormat::AllocText(#s.to_string()),
                });
            }
            AllocMessageValue::Placeholder(n) => {
                let n = *n;
                tokens.extend(quote::quote! {
                    local_fmt::AllocMessageFormat::Placeholder(#n),
                });
            }
            AllocMessageValue::AllocTextIdent(ident) => {
                tokens.extend(quote::quote! {
                    local_fmt::AllocMessageFormat::AllocText(#ident),
                });
            }
        }
    }
}
