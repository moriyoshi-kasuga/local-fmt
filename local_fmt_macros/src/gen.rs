use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use toml::{Table, Value};

use crate::args::Args;

macro_rules! unwrap_err {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Err(syn::Error::new(Span::call_site(), err.to_string())),
        }
    };
}

macro_rules! gen_err_with_str {
    ($s:literal) => {
        Err(syn::Error::new(Span::call_site(), $s))
    };
    ($s:literal, $($arg:tt),+) => {
        Err(syn::Error::new(Span::call_site(), format!($s, $($arg),+)))
    };
}

pub(crate) fn gen_code(macro_args: Args) -> syn::Result<TokenStream> {
    let cargo_dir = unwrap_err!(std::env::var("CARGO_MANIFEST_DIR"));
    let path = std::path::PathBuf::from(cargo_dir).join(&macro_args.locales_path);

    let vis = macro_args.vis.clone();
    let ident = macro_args.ident.clone();
    let lang = macro_args.lang.clone();
    let key = macro_args.key.clone();

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
        call.args.push(match macro_args.fallback {
            Some(ref expr) => expr.clone(),
            None => syn::Expr::Call(syn::ExprCall {
                attrs: Vec::new(),
                func: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: {
                        let mut p = syn::Path::from(syn::Ident::new("Default", Span::call_site()));
                        p.segments
                            .push(syn::Ident::new("default", Span::call_site()).into());
                        p
                    },
                })),
                paren_token: syn::token::Paren(Span::call_site()),
                args: Default::default(),
            }),
        });
        #[cfg(feature = "selected")]
        call.args.push(macro_args.selected.clone());
        #[cfg(feature = "global")]
        call.args.push(macro_args.global.clone());

        call
    };

    let app = path.join("app.toml");

    let gen = if unwrap_err!(std::fs::exists(&app)) {
        let s = unwrap_err!(std::fs::read_to_string(&app));
        let table = unwrap_err!(s.parse::<Table>());
        gen_code_of_app(table, macro_args)?
    } else {
        gen_code_of_table(path, macro_args)?
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

fn gen_code_of_app(table: Table, args: Args) -> syn::Result<TokenStream> {
    let Args {
        locales_path: _,
        vis,
        ident,
        lang,
        key,
        fallback: _,
        #[cfg(feature = "selected")]
        selected,
        #[cfg(feature = "global")]
        global,
    } = args;

    fn def_local(
        def_token: &mut TokenStream,
        path: Vec<&str>,
        last: &str,
        table: &Table,
    ) -> syn::Result<()> {
        let is_table = table.values().all(Value::is_table);
        let is_string = table.values().all(Value::is_str);

        match (is_table, is_string) {
            (true, true) => return gen_err_with_str!("key {} is both table and string", last),
            (false, false) => return gen_err_with_str!("key {} is not table and not string", last),
            (true, false) => {
                let mut table = table.iter();
                for (key, value) in table {
                    let Some(table) = value.as_table() else {
                        return gen_err_with_str!("key {} is not table", key);
                    };
                    def_local(
                        def_token,
                        {
                            let mut path = path.clone();
                            path.push(last);
                            path
                        },
                        &key,
                        table,
                    )?;
                }
            }
            (false, true) => {}
        }

        Ok(())
    }

    let mut def_token = TokenStream::new();

    for (key, value) in &table {
        let Some(table) = value.as_table() else {
            return gen_err_with_str!("key {} is not table", key);
        };
        def_local(&mut def_token, vec![], &key, table)?;
    }

    let gen = quote! {
        let fallback = fmt.fallback;
        macro_rules! is_definitioned_fallback {
            ($key:expr, $locales:expr) => {
                assert!(locales.contains_key(fallback), "key is not found: {} in fallback locale", $key);
                fmt.add_langs_of_key($key, $locale);
            }
        }
        #def_token
    };

    Ok(gen)
}

fn gen_code_of_table(path: PathBuf, args: Args) -> syn::Result<TokenStream> {
    let Args {
        locales_path,
        vis,
        ident,
        lang,
        key,
        fallback: _,
        #[cfg(feature = "selected")]
        selected,
        #[cfg(feature = "global")]
        global,
    } = args;

    let gen = quote! {};

    Ok(gen)
}
