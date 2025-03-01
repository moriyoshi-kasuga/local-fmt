mod args;
mod internal;
mod messages;

#[proc_macro]
pub fn def_local_fmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as args::Args);

    internal::generate(args)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
