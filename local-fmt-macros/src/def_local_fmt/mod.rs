#![allow(clippy::panic)]

mod arg;
mod internal;

pub(crate) use arg::Args;
use proc_macro2::TokenStream;

pub(crate) fn generate(args: Args) -> syn::Result<TokenStream> {
    let internal_token = internal::generate(args.file_type, args.path, &args.message);
    let name = args.name;
    let lang = args.lang;
    let message = args.message.ty;
    let supplier = args.supplier;
    let token = quote::quote! {
        pub const #name: local_fmt::LocalFmt<#lang, #message, {<#lang as enum_table::Enumable>::COUNT}> = {
            use local_fmt::{check_const_message_arg, gen_const_message, ConstMessage, macros::CheckConstMessageArg};

            let messages = enum_table::et!(#lang, #message, |lang| match lang {
                #internal_token,
                #[allow(unreachable_patterns, clippy::panic)]
                _ => panic!("Not filled all languages"),
            });
            local_fmt::LocalFmt::new(messages, #supplier)
        };
    };

    Ok(token)
}
