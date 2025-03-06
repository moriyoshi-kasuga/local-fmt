use proc_macro2::TokenStream;
use syn::Ident;

use crate::parse::MessageToken;

use super::MessageField;

pub(crate) struct LangMessage {
    pub lang: String,
    pub messages: Vec<Message>,
}

pub(crate) struct Message {
    pub key: String,
    pub value: MessageValue,
}

pub(crate) enum MessageValue {
    Token(MessageToken),
    Nested(Vec<Message>),
}

impl LangMessage {
    pub(crate) fn to_token(&self, field: &MessageField) -> TokenStream {
        let lang = Ident::new(&self.lang, proc_macro2::Span::call_site());
        let message = self.messages.iter().map(|v| v.to_token(&self.lang, field));
        let ty = &field.ty;
        quote::quote! {
            #lang => #ty {
                #(
                    #message,
                )*
            }
        }
    }
}

impl Message {
    fn to_token(&self, lang: &str, field: &MessageField) -> TokenStream {
        let name = &self.key;
        let ident = Ident::new(&self.key, proc_macro2::Span::call_site());
        match &field.fields {
            None => {
                todo!();
            }
            Some(fields) => {
                todo!();
            }
        }
    }
}
