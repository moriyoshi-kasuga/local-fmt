use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use toml::Table;

use crate::args::Args;

macro_rules! unwrap_err {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Err(syn::Error::new(Span::call_site(), err.to_string())),
        }
    };
}

macro_rules! unwrap_err_str {
    ($e:expr, $s:literal) => {
        match $e {
            Ok(v) => v,
            Err(_) => return Err(syn::Error::new(Span::call_site(), $s)),
        }
    };
}

pub(crate) fn gen_code(path: PathBuf, args: Args) -> syn::Result<TokenStream> {
    let Args {
        locales_path: _,
        vis,
        ident,
        lang,
        key,
        #[cfg(feature = "selected")]
        selected,
        #[cfg(feature = "global")]
        global,
    } = args.clone();

    // local_fmt::LocalFmt<Lang, Key>::new()
    let init: syn::ExprCall = {
        let mut call = syn::ExprCall {
            attrs: Vec::new(),
            func: Box::new(syn::Expr::Path(syn::ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: {
                    let mut p = syn::Path::from(syn::Ident::new("local_fmt", Span::call_site()));
                    p.segments
                        .push(syn::Ident::new("LocalFmt", Span::call_site()).into());
                    let new = {
                        let mut new: syn::PathSegment =
                            syn::Ident::new("new", Span::call_site()).into();
                        new.arguments = syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: Some(syn::Token![::](Span::call_site())),
                                lt_token: syn::Token![<](Span::call_site()),
                                args: {
                                    let mut args = syn::punctuated::Punctuated::new();
                                    args.push(syn::GenericArgument::Type(syn::Type::Verbatim(
                                        lang.to_token_stream(),
                                    )));
                                    args.push(syn::GenericArgument::Type(syn::Type::Verbatim(
                                        key.to_token_stream(),
                                    )));
                                    args
                                },
                                gt_token: syn::Token![>](Span::call_site()),
                            },
                        );
                        new
                    };
                    p.segments.push(new);
                    p
                },
            })),
            paren_token: syn::token::Paren(Span::call_site()),
            args: Default::default(),
        };
        call.args.push(unwrap_err_str!(
            syn::parse_str("Default::default()"),
            "definition error please report issue"
        ));
        #[cfg(feature = "selected")]
        call.args.push(selected);
        #[cfg(feature = "global")]
        call.args.push(syn::Expr::Path(global));

        call
    };

    let app = path.join("app.toml");

    let gen = if unwrap_err!(std::fs::exists(&app)) {
        let s = unwrap_err!(std::fs::read_to_string(&app));
        let table = unwrap_err!(s.parse::<Table>());
        gen_code_of_app(table, args)?
    } else {
        gen_code_of_table(path, args)?
    };

    let token = quote! {
        #vis static #ident: std::cell::LazyCell<local_fmt::LocalFmt<#lang, #key>> = std::cell::LazyCell::new(|| {
            let mut fmt = #init;
            #gen
            fmt
        });
    };

    Ok(token)
}

fn gen_code_of_app(table: toml::Table, args: Args) -> syn::Result<TokenStream> {
    let Args {
        locales_path: _,
        vis,
        ident,
        lang,
        key,
        #[cfg(feature = "selected")]
        selected,
        #[cfg(feature = "global")]
        global,
    } = args;

    let gen = quote! {};

    Ok(gen)
}

fn gen_code_of_table(path: PathBuf, args: Args) -> syn::Result<TokenStream> {
    let Args {
        locales_path,
        vis,
        ident,
        lang,
        key,
        #[cfg(feature = "selected")]
        selected,
        #[cfg(feature = "global")]
        global,
    } = args;

    let gen = quote! {};

    Ok(gen)
}
