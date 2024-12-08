pub(crate) mod args;
pub(crate) mod as_locale;
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

#[proc_macro_derive(AsLocal)]
pub fn derive_as_local(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    as_locale::derive_as_local(syn::parse_macro_input!(input as syn::DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
