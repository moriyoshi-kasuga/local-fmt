use proc_macro2::TokenStream;
use std::path::PathBuf;

pub(crate) fn generate(args: Args) -> syn::Result<TokenStream> {
    let messages = crate::messages::generate(args.path);
    let name = args.name;
    let lang = args.lang;
    let message = args.message;
    let supplier = args.supplier;
    let parseable = messages.into_iter().map(|m| m.parseable(&lang, &message));
    let token = quote::quote! {
        pub const #name: local_fmt::LocalFmt<#lang, #message, {<#lang as enum_table::Enumable>::COUNT}> = {
            use local_fmt::{check_const_message_arg, gen_const_message, ConstMessage, derive::CheckConstMessageArg};

            let messages = enum_table::et!(#lang, #message, |lang| match lang {
                #(
                    #parseable,
                )*
                #[allow(unreachable_patterns, clippy::panic)]
                _ => panic!("Not filled all languages"),
            });
            local_fmt::LocalFmt::new(messages, #supplier)
        };
    };

    Ok(token)
}

pub(crate) struct Args {
    pub(crate) name: syn::Ident,
    pub(crate) lang: syn::Ident,
    pub(crate) message: syn::Ident,
    pub(crate) supplier: syn::Expr,
    pub(crate) path: ArgPath,
}

pub(crate) enum ArgPath {
    File(PathBuf),
    Folder(PathBuf),
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        mod kw {
            syn::custom_keyword!(name);
            syn::custom_keyword!(lang);
            syn::custom_keyword!(message);
            syn::custom_keyword!(static_supplier);
            syn::custom_keyword!(dynamic_supplier);
            syn::custom_keyword!(lang_file);
            syn::custom_keyword!(lang_folder);
        }

        macro_rules! parse {
            ($ident:ident) => {
                parse!($ident, syn::Ident);
            };
            ($ident:ident, $ty:ty) => {
                parse!($ident, $ty, without_comma);
                let _: syn::Token![,] = input.parse()?;
            };
            ($ident:ident, $ty:ty, without_comma) => {
                let _: kw::$ident = input.parse()?;
                let _: syn::Token![=] = input.parse()?;
                let $ident: $ty = input.parse()?;
            };
        }

        parse!(name);
        parse!(lang);
        parse!(message);

        let supplier = if input.peek(kw::static_supplier) {
            parse!(static_supplier, syn::Expr);
            syn::parse_quote!(local_fmt::LangSupplier::Static(#static_supplier))
        } else if input.peek(kw::dynamic_supplier) {
            parse!(dynamic_supplier, syn::Expr);
            syn::parse_quote!(local_fmt::LangSupplier::Dynamic(#dynamic_supplier))
        } else {
            return Err(input.error("expected static_supplier or dynamic_supplier"));
        };

        #[allow(clippy::panic)]
        let crate_root = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| panic!("failed to get CARGO_MANIFEST_DIR"));
        let crate_root = PathBuf::from(crate_root);

        let path = if input.peek(kw::lang_file) {
            parse!(lang_file, syn::LitStr, without_comma);

            ArgPath::File(crate_root.join(lang_file.value()))
        } else if input.peek(kw::lang_folder) {
            parse!(lang_folder, syn::LitStr, without_comma);

            ArgPath::Folder(crate_root.join(lang_folder.value()))
        } else {
            return Err(input.error("expected lang_file or lang_folder"));
        };

        if input.peek(syn::Token![,]) {
            let _ = input.parse::<syn::Token![,]>()?;
        }

        Ok(Self {
            name,
            lang,
            message,
            supplier,
            path,
        })
    }
}
