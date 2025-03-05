use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::{parse::Parse, LitStr};

use crate::parse::MessageToken;

pub(crate) struct Args {
    pub(crate) is_unchecked: bool,
    pub(crate) text: LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        mod kw {
            syn::custom_keyword!(unchecked);
        }
        if input.peek(LitStr) {
            Ok(Args {
                is_unchecked: false,
                text: input.parse()?,
            })
        } else if input.peek(kw::unchecked) {
            let _ = input.parse::<kw::unchecked>()?;
            let _ = input.parse::<syn::Token![,]>()?;
            let text = input.parse()?;
            Ok(Args {
                is_unchecked: true,
                text,
            })
        } else {
            Err(input.error("expected string literal or unchecked"))
        }
    }
}

impl Args {
    pub(crate) fn to_token(&self, is_static: bool) -> syn::Result<TokenStream> {
        let token = if self.is_unchecked {
            MessageToken::from_str_unchecked(&self.text.value())
                .map_err(|v| syn::Error::new(self.text.span(), v))?
        } else {
            MessageToken::from_str(&self.text.value())
                .map_err(|v| syn::Error::new(self.text.span(), v))?
        };

        if is_static {
            Ok(token.to_static_token_stream())
        } else {
            Ok(token.to_vec_token_stream())
        }
    }
}
