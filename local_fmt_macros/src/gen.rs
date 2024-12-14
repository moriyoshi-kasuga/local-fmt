use std::{collections::HashMap, path::PathBuf};

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
        let selected = macro_args.selected.clone();
        #[cfg(feature = "global")]
        let global = macro_args.global.clone();

        {
            let def = if let Some(ref app_file) = macro_args.app_file {
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

            let mut gen = TokenStream::new();

            for (lang, locales) in def {
                let (keys, v): (Vec<_>, Vec<_>) = locales.into_iter().unzip();

                let keys = quote! {
                    #lang => generate_match!({ #( #keys => #v ,)* }, "key \"{1}\" is not defined for lang \"{0}\"", #lang),
                };

                gen.extend(keys);
            }

            let gen = quote! {
                generate_match!({ #gen }, "Lang \"{}\" is not defined")
            };

            match syn::parse2::<syn::Expr>(gen) {
                Ok(gen) => call.args.push(gen),
                Err(_) => return gen_err_with_str!("failed builtin"),
            };
        }

        #[cfg(feature = "selected")]
        call.args.push(selected.clone());
        #[cfg(feature = "global")]
        call.args.push(global.clone());

        call
    };

    let token = quote! {
        #[allow(clippy::panic, clippy::expect_used)]
        #vis static #ident: std::sync::LazyLock<local_fmt::LocalFmt<#lang, #key>> = std::sync::LazyLock::new(|| {
            macro_rules! generate_match {
                ( { $($key:expr => $variant:expr,)+ }, $($arg:tt)+ ) => {
                    local_fmt::EnumableMap::new(|v| match v {
                        $(__key__ if Into::<&'static str>::into(__key__) == $key => $variant,)+
                        __key__ => panic!($($arg)+, __key__),
                    })
                }
            }

            #init
        });
    };

    Ok(token)
}

fn gen_code_of_app(
    table: Table,
    args: Args,
) -> syn::Result<HashMap<String, HashMap<String, String>>> {
    let Args { locales_path, .. } = args;

    fn def_local(
        def: &mut HashMap<String, HashMap<String, String>>,
        path: Vec<&str>,
        table: &Table,
    ) -> syn::Result<()> {
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
                for (lang, s) in table.keys().zip(strings) {
                    def.entry(lang.clone())
                        .or_default()
                        .insert(path.clone(), s.to_string());
                }
                Ok(())
            }
            (Some(tables), None) => {
                for (key, table) in table.keys().zip(tables) {
                    let mut path = path.clone();
                    path.push(key);
                    def_local(def, path, table)?;
                }
                Ok(())
            }
        }
    }

    if table.is_empty() {
        return gen_err_with_str!("app.toml is empty");
    }

    let mut def = HashMap::<String, HashMap<String, String>>::new();

    for (key, value) in &table {
        let Some(table) = value.as_table() else {
            return gen_err_with_str!("key {} is not table in {}/app.toml", key, locales_path);
        };
        def_local(&mut def, vec![key], table)?;
    }

    Ok(def)
}

fn gen_code_of_table(
    path: PathBuf,
    args: Args,
) -> syn::Result<HashMap<String, HashMap<String, String>>> {
    let _ = path;
    let Args { .. } = args;

    // let mut def = HashMap::<String, HashMap<String, String>>::new();

    Ok(Default::default())
}
