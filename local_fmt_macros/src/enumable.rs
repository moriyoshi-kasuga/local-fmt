use proc_macro2::TokenStream;

pub(crate) fn derive_enumable(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            derive_enumable_internal(&input, variants.iter())
        }
        _ => Err(syn::Error::new_spanned(
            input,
            "currently only structs with named fields are supported",
        )),
    }
}

fn derive_enumable_internal<'a>(
    input: &syn::DeriveInput,
    variatns: impl Iterator<Item = &'a syn::Variant>,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let field_name = variatns.map(|f| &f.ident).collect::<Vec<_>>();
    let len = field_name.len();
    let field_id = Vec::from_iter(0..len);
    let token = quote::quote! {
        impl #impl_generics local_fmt::Enumable for #ident #ty_generics #where_clause {
            type Array<V> = [V; #len];

            fn _from_usize(value: usize) -> Self {
                match value {
                    #(#field_id => #ident::#field_name,)*
                    #[allow(clippy::panic)]
                    _ => panic!("cannot convert {} to {}", value, stringify!(#ident)),
                }
            }

            fn _into_usize(self) -> usize {
                match self {
                    #(#ident::#field_name => #field_id),*
                }
            }
        }
    };
    Ok(token)
}
