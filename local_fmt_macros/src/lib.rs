pub(crate) mod args;
pub(crate) mod convert_str;
pub(crate) mod gen;

use args::Args;
use gen::gen_code;

#[proc_macro]
#[allow(clippy::expect_used)]
pub fn def_local_fmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_code(syn::parse_macro_input!(input as Args))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(ConvertStr)]
pub fn derive_convert_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    convert_str::derive_convert_str(syn::parse_macro_input!(input as syn::DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
