use quote::ToTokens;

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
        let messages = &self.messages;
        let token = quote::quote! {
            #lang_ident::#current_lang => #mesaages_ident {
                #(#messages,)*
            }
        };
        tokens.extend(token);
    }
}

impl ToTokens for Message {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let values = &self.values;
        let token = quote::quote! {
            #name: local_fmt::gen_const_message!(#(#values),*)
        };
        tokens.extend(token);
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
