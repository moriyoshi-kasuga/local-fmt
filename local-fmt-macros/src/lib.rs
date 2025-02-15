pub(crate) mod args;
pub(crate) mod gen;

#[proc_macro]
pub fn def_local_fmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen::gen_code(syn::parse_macro_input!(input as args::Args))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
