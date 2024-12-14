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

macro_rules! gen_err_with_str {
    ($s:literal) => {
        Err(syn::Error::new(Span::call_site(), $s))
    };
    ($s:literal, $($arg:tt)+) => {
        Err(syn::Error::new(Span::call_site(), format!($s, $($arg)+)))
    };
}

pub(crate) fn gen_code(macro_args: Args) -> syn::Result<TokenStream> {
    let cargo_dir = unwrap_err!(std::env::var("CARGO_MANIFEST_DIR"));
    let path = std::path::PathBuf::from(cargo_dir).join(&macro_args.locales_path);
    if !unwrap_err!(std::fs::exists(&path)) {
        return Err(syn::Error::new(
            Span::call_site(),
            format!("locales path {} is not found", path.display()),
        ));
    }

    let vis = macro_args.vis.clone();
    let ident = macro_args.ident.clone();
    let lang = macro_args.lang.clone();
    let key = macro_args.key.clone();

    // local_fmt::LocalFmt<Lang, Key>::new()
    let init: syn::ExprCall = {
        #[allow(unused_mut)]
        let mut call = syn::ExprCall {
            attrs: Vec::new(),
            func: Box::new(syn::Expr::Path(syn::ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: {
                    let mut p = syn::Path::from(syn::Ident::new("local_fmt", Span::call_site()));
                    let mut def: syn::PathSegment =
                        syn::Ident::new("LocalFmt", Span::call_site()).into();

                    def.arguments =
                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
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
                        });

                    p.segments.push(def);

                    p.segments
                        .push(syn::Ident::new("new", Span::call_site()).into());
                    p
                },
            })),
            paren_token: syn::token::Paren(Span::call_site()),
            args: Default::default(),
        };
        #[cfg(feature = "selected")]
        call.args.push(macro_args.selected.clone());
        #[cfg(feature = "global")]
        call.args.push(macro_args.global.clone());

        call
    };

    let gen = if let Some(ref app_file) = macro_args.app_file {
        let app = path.join(app_file);
        if !unwrap_err!(std::fs::exists(&app)) {
            return gen_err_with_str!("app file {} is not found", app.display());
        };
        let s = unwrap_err!(std::fs::read_to_string(&app));
        let table = unwrap_err!(s.parse::<Table>());
        gen_code_of_app(table, macro_args)?
    } else {
        gen_code_of_table(path, macro_args)?
    };

    let token = quote! {
        #vis static #ident: std::sync::LazyLock<local_fmt::LocalFmt<#lang, #key>> = std::sync::LazyLock::new(|| {
            use std::collections::HashMap;
            let mut fmt = #init;
            #gen
            fmt
        });
    };

    Ok(token)
}

fn gen_code_of_app(table: Table, args: Args) -> syn::Result<TokenStream> {
    let Args {
        locales_path,
        lang,
        key,
        ..
    } = args;

    fn def_local(def_token: &mut TokenStream, path: Vec<&str>, table: &Table) -> syn::Result<()> {
        let langs: Vec<&String> = table.keys().collect();
        let tables: Option<Vec<&Table>> = table.values().map(|v| v.as_table()).collect();
        let strings: Option<Vec<&str>> = table.values().map(|v| v.as_str()).collect();

        match (tables, strings) {
            (None, None) => {
                gen_err_with_str!("key {} is not table and not string", path.join("."))
            }
            (Some(_), Some(_)) => {
                gen_err_with_str!("key {} is both table and string", path.join("."))
            }
            (None, Some(strings)) => {
                let path = path.join("_");
                let token = quote! {
                    {
                        let mut locales = HashMap::with_capacity(capacity);
                        let path = #path.try_into().unwrap();
                        #(
                            locales.insert(to_lang!(#langs,path), #strings);
                        )*
                        is_definitioned!(path, locales);
                    }
                };
                def_token.extend(token);
                Ok(())
            }
            (Some(tables), None) => {
                for (key, table) in langs.iter().zip(tables) {
                    let mut path = path.clone();
                    path.push(key);
                    def_local(def_token, path, table)?;
                }
                Ok(())
            }
        }
    }

    let mut def_token = TokenStream::new();

    if table.is_empty() {
        return gen_err_with_str!("app.toml is empty");
    }

    for (key, value) in &table {
        let Some(table) = value.as_table() else {
            return gen_err_with_str!("key {} is not table in {}/app.toml", key, locales_path);
        };
        def_local(&mut def_token, vec![key], table)?;
    }

    let gen = quote! {
        let langs = <#lang as local_fmt::EnumIter>::iter();
        let capacity = langs.len();
        let keys = <#key as local_fmt::EnumIter>::iter();

        let mut def_keys = std::collections::HashSet::<#key>::new();

        macro_rules! is_definitioned {
            ($key:expr, $locales:expr) => {
                assert_eq!(capacity, $locales.len(), "Not all locales are defined for key \"{}\"", $key);
                for lang in langs.clone() {
                    assert!($locales.contains_key(lang), "Not all locales are defined for key \"{}\" and lang \"{}\"", $key, lang);
                }
                assert!(def_keys.insert($key), "Key \"{}\" is already defined", $key);
                fmt.add_langs_of_key($key, $locales);
            }
        }

        macro_rules! to_lang {
            ($lang:expr,$path:expr) => {
                $lang.try_into().map_err(|err| format!("happen at \"{}\": {}", $path, err)).unwrap()
            }
        }

        #def_token

        assert_eq!(keys.len(), def_keys.len(), "Not all keys are defined");
        for key in keys {
            assert!(def_keys.contains(key), "Key \"{}\" is not defined", key);
        }
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
        ..
    } = args;

    let gen = quote! {};

    Ok(gen)
}
