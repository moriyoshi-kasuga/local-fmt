use syn::{Expr, Ident, LitStr, Path, Token, Visibility};

#[derive(Clone)]
pub(crate) struct Args {
    pub locales_path: String,
    pub vis: Visibility,
    pub ident: Ident,
    pub lang: Ident,
    pub key: Ident,
    pub fallback: Option<Expr>,
    #[cfg(feature = "selected")]
    pub selected: Expr,
    #[cfg(feature = "global")]
    pub global: Expr,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut locales_path = None;
        let mut vis = None;
        let mut ident = None;
        let mut lang = None;
        let mut key = None;
        let mut fallback = None;

        #[cfg(feature = "selected")]
        let mut selected = None;

        #[cfg(feature = "global")]
        let mut global = None;

        loop {
            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>();
            }

            if let Ok(path) = input.parse::<Path>() {
                match path {
                    path if path.is_ident("path") => {
                        if locales_path.is_some() {
                            return Err(syn::Error::new_spanned(path, "duplicate path attribute"));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let lit_str = input.parse::<LitStr>()?;
                        locales_path = Some(lit_str.value());
                    }
                    path if path.is_ident("visibility") => {
                        if vis.is_some() {
                            return Err(syn::Error::new_spanned(
                                path,
                                "duplicate visibility attribute",
                            ));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let v = input.parse::<Visibility>()?;
                        vis = Some(v);
                    }
                    path if path.is_ident("ident") => {
                        if ident.is_some() {
                            return Err(syn::Error::new_spanned(path, "duplicate ident attribute"));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let i = input.parse::<Ident>()?;
                        ident = Some(i);
                    }
                    path if path.is_ident("lang") => {
                        if lang.is_some() {
                            return Err(syn::Error::new_spanned(path, "duplicate lang attribute"));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let i = input.parse::<Ident>()?;
                        lang = Some(i);
                    }
                    path if path.is_ident("key") => {
                        if key.is_some() {
                            return Err(syn::Error::new_spanned(path, "duplicate key attribute"));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let i = input.parse::<Ident>()?;
                        key = Some(i);
                    }
                    path if path.is_ident("fallback") => {
                        if fallback.is_some() {
                            return Err(syn::Error::new_spanned(
                                path,
                                "duplicate fallback attribute",
                            ));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let expr = input.parse::<Expr>()?;
                        fallback = Some(expr);
                    }
                    #[cfg(feature = "selected")]
                    path if path.is_ident("selected") => {
                        if selected.is_some() {
                            return Err(syn::Error::new_spanned(
                                path,
                                "duplicate selected attribute",
                            ));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let expr = input.parse::<Expr>()?;
                        selected = Some(expr);
                    }
                    #[cfg(feature = "global")]
                    path if path.is_ident("global") => {
                        if global.is_some() {
                            return Err(syn::Error::new_spanned(
                                path,
                                "duplicate global attribute",
                            ));
                        }
                        let _ = input.parse::<Token![=]>()?;

                        let path = input.parse::<Expr>()?;
                        global = Some(path);
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(path, "unknown attribute"));
                    }
                }
            } else {
                break;
            }
        }

        Ok(Self {
            locales_path: locales_path.unwrap_or("locales".to_string()),
            vis: vis.unwrap_or(Visibility::Inherited),
            ident: ident.ok_or_else(|| syn::Error::new(input.span(), "no ident attribute"))?,
            lang: lang.ok_or_else(|| syn::Error::new(input.span(), "no lang attribute"))?,
            key: key.ok_or_else(|| syn::Error::new(input.span(), "no key attribute"))?,
            fallback,
            #[cfg(feature = "selected")]
            selected: selected
                .ok_or_else(|| syn::Error::new(input.span(), "no selected attribute"))?,
            #[cfg(feature = "global")]
            global: global.ok_or_else(|| syn::Error::new(input.span(), "no global attribute"))?,
        })
    }
}
