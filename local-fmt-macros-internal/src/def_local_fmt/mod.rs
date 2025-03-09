#![allow(clippy::panic)]

mod arg;
mod internal;

pub use arg::Args;
use proc_macro2::TokenStream;

pub fn generate(args: Args) -> syn::Result<TokenStream> {
    let lang_messages = internal::generate(args.file_type, args.path, &args.message);
    let internal_tokens = lang_messages
        .iter()
        .map(|lang_message| lang_message.to_token(&args.message))
        .collect::<Vec<_>>();
    let name = args.name;
    let lang = args.lang;
    let message = args.message.ty;
    let supplier = args.supplier;
    let token = quote::quote! {
        pub const #name: local_fmt::LocalFmt<#lang, #message, {<#lang as enum_table::Enumable>::COUNT}> = {
            use local_fmt::macros::CheckConstMessageArg;

            let messages = enum_table::et!(#lang, #message, |lang| match lang {
                #(
                    #lang::#internal_tokens,
                )*
                #[allow(unreachable_patterns, clippy::panic)]
                _ => panic!("Not filled all languages"),
            });
            local_fmt::LocalFmt::new(messages, #supplier)
        };
    };

    Ok(token)
}
