pub(crate) mod args;
pub(crate) mod convert_str;
pub(crate) mod gen;

use args::Args;
use gen::gen_code;

#[proc_macro]
#[allow(clippy::expect_used)]
pub fn def_local_fmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty");

    let args = syn::parse_macro_input!(input as Args);

    let locales_path = std::path::PathBuf::from(cargo_dir).join(&args.locales_path);

    gen_code(locales_path, args)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(ConvertStr)]
pub fn derive_convert_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    convert_str::derive_convert_str(syn::parse_macro_input!(input as syn::DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
