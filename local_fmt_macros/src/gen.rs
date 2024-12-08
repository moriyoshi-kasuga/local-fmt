use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
use quote::quote;
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

    let init: syn::ExprMethodCall = {
        let def: syn::Expr = unwrap_err_str!(
            syn::parse_str("local_fmt::LocalFmt::new()"),
            "definition error please report issue"
        );
        let syn::Expr::MethodCall(mut call) = def else {
            return Err(syn::Error::new(
                Span::call_site(),
                "definition error please report issue",
            ));
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
