use quote::ToTokens;
use syn::Ident;

use super::MessageValue;

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
