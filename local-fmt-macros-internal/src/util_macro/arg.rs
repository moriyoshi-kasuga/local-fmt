use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::{parse::Parse, punctuated::Punctuated, spanned::Spanned, LitStr};

use crate::parse::MessageToken;

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
        let token =
            MessageToken::from_str(&text).map_err(|v| syn::Error::new(self.texts.span(), v))?;

        if is_static {
            Ok(token.to_static_token_stream())
        } else {
            Ok(token.to_alloc_token_stream())
        }
    }
}
