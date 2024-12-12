use proc_macro2::TokenStream;

pub(crate) fn derive_enum_iter(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            derive_enum_iter_internal(&input, variants.iter())
        }
        _ => Err(syn::Error::new_spanned(
            input,
            "currently only structs with named fields are supported",
        )),
    }
}

fn derive_enum_iter_internal<'a>(
    input: &syn::DeriveInput,
    variatns: impl Iterator<Item = &'a syn::Variant>,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let field_name = variatns.map(|f| &f.ident).collect::<Vec<_>>();

    let token = quote::quote! {
        impl #impl_generics local_fmt::EnumIter for #ident #ty_generics #where_clause {
            fn iter() -> impl Iterator<Item = Self> {
                [#(Self::#field_name),*].into_iter()
            }
        }
    };
    Ok(token)
}
