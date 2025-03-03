use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

pub(crate) struct LangMessage {
    pub lang: String,
    pub messages: Vec<Message>,
}

pub(crate) struct Message {
    pub name: String,
    pub values: Vec<MessageValue>,
}

pub(crate) enum MessageValue {
    Text(String),
    Placeholder(usize),
}

impl LangMessage {
    pub(crate) fn parseable<'a>(
        self,
        lang_ident: &'a syn::Ident,
        mesaages_ident: &'a syn::Ident,
    ) -> ParseableLangMessage<'a> {
        let current_lang = syn::Ident::new(&self.lang, proc_macro2::Span::call_site());
        ParseableLangMessage {
            lang_ident,
            current_lang,
            mesaages_ident,
            messages: self.messages,
        }
    }
}

pub(crate) struct ParseableLangMessage<'a> {
    pub lang_ident: &'a syn::Ident,
    pub current_lang: syn::Ident,
    pub mesaages_ident: &'a syn::Ident,
    pub messages: Vec<Message>,
}

impl ToTokens for ParseableLangMessage<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let lang_ident = self.lang_ident;
        let current_lang = &self.current_lang;
        let mesaages_ident = self.mesaages_ident;
        let current_lang_str = self.current_lang.to_string();
        let messages = self.messages.iter().fold(TokenStream::new(), |mut acc, m| {
            let token = m.to_token(&current_lang_str);
            let token = quote::quote! {
                #token,
            };
            acc.extend(token);
            acc
        });

        let token = quote::quote! {
            #lang_ident::#current_lang => #mesaages_ident {
                #messages
            }
        };
        tokens.extend(token);
    }
}

impl Message {
    fn to_token(&self, current_lang: &str) -> TokenStream {
        let name = &self.name;
        let ident = Ident::new(&self.name, proc_macro2::Span::call_site());
        let values = &self.values;

        let arg_count = values
            .iter()
            .filter(|v| matches!(v, MessageValue::Placeholder(_)))
            .count();

        let values = values.iter().fold(TokenStream::new(), |mut acc, v| {
            match v {
                MessageValue::Text(text) => {
                    acc.extend(quote::quote! {
                        #text,
                    });
                }
                MessageValue::Placeholder(index) => {
                    acc.extend(quote::quote! {
                        { #index },
                    });
                }
            }
            acc
        });

        let token = quote::quote! {
            #ident: check_const_message_arg!(#current_lang, #name, #arg_count, #values)
        };

        token
    }
}

impl ToTokens for MessageValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            MessageValue::Text(text) => {
                tokens.extend(quote::quote! {
                    #text
                });
            }
            MessageValue::Placeholder(index) => {
                tokens.extend(quote::quote! {
                    { #index }
                });
            }
        }
    }
}
