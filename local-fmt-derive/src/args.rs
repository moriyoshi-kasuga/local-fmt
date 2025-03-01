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
            syn::custom_keyword!(supplier);
            syn::custom_keyword!(lang_file);
            syn::custom_keyword!(lang_folder);
        }

        macro_rules! parse {
            ($ident:ident) => {
                parse!($ident, syn::Ident);
            };
            ($ident:ident, $ty:ty) => {
                let _: kw::$ident = input.parse()?;
                let _: syn::Token![=] = input.parse()?;
                let $ident: $ty = input.parse()?;
                let _: syn::Token![,] = input.parse()?;
            };
        }

        parse!(name);
        parse!(lang);
        parse!(message);
        parse!(supplier, syn::Expr);

        let path = if input.peek(kw::lang_file) {
            parse!(lang_file, syn::LitStr);

            ArgPath::File(lang_file.value().into())
        } else if input.peek(kw::lang_folder) {
            parse!(lang_folder, syn::LitStr);

            ArgPath::Folder(lang_folder.value().into())
        } else {
            return Err(input.error("expected lang_file or lang_folder"));
        };

        let _ = input.parse::<syn::Token![,]>();

        Ok(Self {
            name,
            lang,
            message,
            supplier,
            path,
        })
    }
}
