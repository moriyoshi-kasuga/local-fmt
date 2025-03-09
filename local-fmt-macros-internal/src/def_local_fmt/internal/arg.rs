use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    parse::{MessageToken, MessageTokenValue},
    utils::hierarchy::Hierarchy,
};

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
        let message = self
            .messages
            .iter()
            .map(|v| v.to_token(&self.lang, &mut Hierarchy::new(), field));
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

fn message_token_to_token_stream(token: &MessageToken) -> TokenStream {
    match token.placeholder_max {
        Some(_) => token.to_static_token_stream(),
        None => {
            let value = token.values.iter().fold(String::new(), |mut acc, v| {
                match v {
                    MessageTokenValue::StaticText(v) => acc.push_str(v),
                    MessageTokenValue::PlaceholderArg(_) => {
                        unreachable!()
                    }
                    MessageTokenValue::PlaceholderIdent(_) => {
                        unreachable!()
                    }
                }
                acc
            });

            quote::quote! {
                #value
            }
        }
    }
}

impl Message {
    fn to_token(
        &self,
        lang: &str,
        hierarchy: &mut Hierarchy<String>,
        field: &MessageField,
    ) -> TokenStream {
        let name = &self.key;
        let ident = Ident::new(&self.key, proc_macro2::Span::call_site());
        match &field.fields {
            None => match &self.value {
                MessageValue::Token(token) => {
                    let value = message_token_to_token_stream(token);
                    quote::quote! {
                        #ident: CheckConstMessageArg::check(#lang, #name, #value)
                    }
                }
                MessageValue::Nested(messages) => {
                    let mut token_stream = TokenStream::new();
                    for message in messages {
                        let token = hierarchy.process(name.to_string(), |hierarchy| {
                            message.to_token(lang, hierarchy, field)
                        });
                        token_stream.extend(quote::quote! {
                            #ident: #token,
                        });
                    }
                    token_stream
                }
            },
            Some(fields) => match fields.iter().find(|(ty, _)| ty == &ident) {
                None => match &self.value {
                    MessageValue::Token(token) => {
                        let value = message_token_to_token_stream(token);
                        quote::quote! {
                            #ident: CheckConstMessageArg::check(#lang, #name, #value)
                        }
                    }
                    MessageValue::Nested(_) => {
                        panic!(
                            "Expected a string with key {}, but got a nested message in language {}",
                            hierarchy.join(name), lang
                        )
                    }
                },
                Some((ident, field)) => {
                    let message = match self.value {
                        MessageValue::Nested(ref messages) => messages,
                        MessageValue::Token(_) => {
                            panic!(
                                "Expected a nested message with key {}, but got a string in language {}",
                                hierarchy.join(name), lang
                            )
                        }
                    };
                    let token = hierarchy.process(name.to_string(), |hierarchy| {
                        message
                            .iter()
                            .map(|m| m.to_token(lang, hierarchy, field))
                            .collect::<Vec<_>>()
                    });

                    let ty = &field.ty;

                    quote::quote! {
                        #ident: #ty {
                            #(
                                #token,
                            )*
                        }
                    }
                }
            },
        }
    }
}
