use proc_macro2::TokenStream;

pub(crate) fn derive_convert_str(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            derive_convert_str_internal(&input, variants.iter())
        }
        _ => Err(syn::Error::new_spanned(
            input,
            "currently only structs with named fields are supported",
        )),
    }
}

fn derive_convert_str_internal<'a>(
    input: &syn::DeriveInput,
    variatns: impl Iterator<Item = &'a syn::Variant>,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let field_name = variatns.map(|f| &f.ident).collect::<Vec<_>>();
    let name = field_name
        .iter()
        .map(|i| i.to_string().to_lowercase())
        .collect::<Vec<_>>();

    let token = quote::quote! {
        impl #impl_generics From<#ident> for &'static str #ty_generics #where_clause {
            fn from(value: #ident) -> Self {
                match value {
                    #(#ident::#field_name => #name),*
                }
            }
        }

        impl #impl_generics TryFrom<&str> for #ident #ty_generics #where_clause {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    #(#name => Ok(Self::#field_name)),*,
                    _ => Err(format!("cannot convert {} to {}", value, stringify!(#ident))),
                }
            }
        }
    };
    Ok(token)
}
