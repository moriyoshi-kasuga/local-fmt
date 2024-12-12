pub(crate) mod args;
pub(crate) mod convert_str;
pub(crate) mod enum_iter;
pub(crate) mod enumable;
pub(crate) mod gen;

#[proc_macro]
pub fn def_local_fmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen::gen_code(syn::parse_macro_input!(input as args::Args))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(ConvertStr)]
pub fn derive_convert_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    convert_str::derive_convert_str(syn::parse_macro_input!(input as syn::DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enum_iter::derive_enum_iter(syn::parse_macro_input!(input as syn::DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Enumable)]
pub fn derive_enumable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enumable::derive_enumable(syn::parse_macro_input!(input as syn::DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
