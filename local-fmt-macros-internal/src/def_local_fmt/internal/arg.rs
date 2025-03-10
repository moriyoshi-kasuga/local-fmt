use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use crate::{
    parse::{StaticMessage, StaticMessageValue},
    utils::hierarchy::Hierarchy,
};

use super::MessageField;

pub struct LangMessage {
    pub lang: String,
    pub messages: Vec<Message>,
}

pub struct Message {
    pub key: String,
    pub value: MessageValue,
}

pub enum MessageValue {
    Token(StaticMessage),
    Nested(Vec<Message>),
}

impl LangMessage {
    pub fn to_token(&self, field: &MessageField) -> TokenStream {
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

fn message_token_to_token_stream(
    ident: &Ident,
    lang: &str,
    name: &str,
    token: &StaticMessage,
) -> TokenStream {
    let value = match token.placeholder_max {
        Some(_) => token.to_token_stream(),
        None => {
            let value = token.values.iter().fold(String::new(), |mut acc, v| {
                match v {
                    StaticMessageValue::StaticText(v) => acc.push_str(v),
                    _ => {
                        unreachable!()
                    }
                }
                acc
            });
            value.to_token_stream()
        }
    };
    quote::quote! {
        #ident: check_static_message_arg(#lang, #name, &#value)
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
                    message_token_to_token_stream(&ident, lang, &hierarchy.join(name), token)
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
                        message_token_to_token_stream(&ident, lang, &hierarchy.join(name), token)
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
