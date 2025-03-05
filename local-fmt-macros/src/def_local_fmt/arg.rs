use std::path::PathBuf;

pub(crate) struct Args {
    pub(crate) name: syn::Ident,
    pub(crate) lang: syn::Ident,
    pub(crate) message: syn::Ident,
    pub(crate) supplier: syn::Expr,
    pub(crate) path: ArgPath,
}

pub(crate) enum ArgPath {
    File(PathBuf),
    Folder(PathBuf),
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        mod kw {
            syn::custom_keyword!(name);
            syn::custom_keyword!(lang);
            syn::custom_keyword!(message);
            syn::custom_keyword!(static_supplier);
            syn::custom_keyword!(dynamic_supplier);
            syn::custom_keyword!(lang_file);
            syn::custom_keyword!(lang_folder);
        }

        macro_rules! parse {
            ($ident:ident) => {
                parse!($ident, syn::Ident);
            };
            ($ident:ident, $ty:ty) => {
                parse!($ident, $ty, without_comma);
                let _: syn::Token![,] = input.parse()?;
            };
            ($ident:ident, $ty:ty, without_comma) => {
                let _: kw::$ident = input.parse()?;
                let _: syn::Token![=] = input.parse()?;
                let $ident: $ty = input.parse()?;
            };
        }

        parse!(name);
        parse!(lang);
        parse!(message);

        let supplier = if input.peek(kw::static_supplier) {
            parse!(static_supplier, syn::Expr);
            syn::parse_quote!(local_fmt::LangSupplier::Static(#static_supplier))
        } else if input.peek(kw::dynamic_supplier) {
            parse!(dynamic_supplier, syn::Expr);
            syn::parse_quote!(local_fmt::LangSupplier::Dynamic(#dynamic_supplier))
        } else {
            return Err(input.error("expected static_supplier or dynamic_supplier"));
        };

        #[allow(clippy::panic)]
        let crate_root = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| panic!("failed to get CARGO_MANIFEST_DIR"));
        let crate_root = PathBuf::from(crate_root);

        let path = if input.peek(kw::lang_file) {
            parse!(lang_file, syn::LitStr, without_comma);

            ArgPath::File(crate_root.join(lang_file.value()))
        } else if input.peek(kw::lang_folder) {
            parse!(lang_folder, syn::LitStr, without_comma);

            ArgPath::Folder(crate_root.join(lang_folder.value()))
        } else {
            return Err(input.error("expected lang_file or lang_folder"));
        };

        if input.peek(syn::Token![,]) {
            let _ = input.parse::<syn::Token![,]>()?;
        }

        Ok(Self {
            name,
            lang,
            message,
            supplier,
            path,
        })
    }
}
