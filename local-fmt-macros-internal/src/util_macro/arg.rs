use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::{parse::Parse, LitStr};

use crate::parse::MessageToken;

pub struct Args {
    pub text: LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Args {
            text: input.parse()?,
        })
    }
}

impl Args {
    pub fn to_token(&self, is_static: bool) -> syn::Result<TokenStream> {
        let token = MessageToken::from_str(&self.text.value())
            .map_err(|v| syn::Error::new(self.text.span(), v))?;

        if is_static {
            Ok(token.to_static_token_stream())
        } else {
            Ok(token.to_alloc_token_stream())
        }
    }
}
