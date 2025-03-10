use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, spanned::Spanned, LitStr};

use crate::parse::{AllocMessage, StaticMessage};

pub struct Args {
    pub texts: Punctuated<LitStr, syn::Token![,]>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Args {
            texts: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Args {
    pub fn to_token(&self, is_static: bool) -> syn::Result<TokenStream> {
        let text = self.texts.iter().fold(String::new(), |mut acc, lit| {
            acc.push_str(&lit.value());
            acc
        });

        if is_static {
            StaticMessage::from_str(&text)
                .map_err(|v| syn::Error::new(self.texts.span(), v))
                .map(|v| v.into_token_stream())
        } else {
            AllocMessage::from_str(&text)
                .map_err(|v| syn::Error::new(self.texts.span(), v))
                .map(|v| v.into_token_stream())
        }
    }
}
