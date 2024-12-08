use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
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

pub(crate) fn gen_code(path: PathBuf, args: Args) -> syn::Result<TokenStream> {
    let app = path.join("app.toml");

    if unwrap_err!(std::fs::exists(&app)) {
        let s = unwrap_err!(std::fs::read_to_string(path));
        let table = unwrap_err!(s.parse::<Table>());
        gen_code_of_app(table, args)
    } else {
        todo!()
    }
}

fn gen_code_of_app(table: toml::Table, args: Args) -> syn::Result<TokenStream> {
    let token = TokenStream::new();
    Ok(token)
}

fn gen_code_of_table(path: PathBuf, args: Args) -> syn::Result<TokenStream> {
    todo!()
}
