use std::path::PathBuf;

use syn::parse::ParseStream;
use syn::{Ident, LitStr};

#[derive(Debug)]
pub struct MessageField {
    pub ty: Ident,
    pub fields: Option<Vec<(Ident, MessageField)>>,
}

impl syn::parse::Parse for MessageField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Ident = input.parse()?;
        if !input.peek(syn::token::Brace) {
            if !input.is_empty() {
                let _: syn::Token![,] = input.parse()?;
            }
            return Ok(Self { ty, fields: None });
        };
        let content;
        syn::braced!(content in input);

        let mut fields = Vec::new();
        while !content.is_empty() {
            let ty: Ident = content.parse()?;

            let _: syn::Token![:] = content.parse()?;

            let field: MessageField = content.parse()?;

            fields.push((ty, field));

            if content.is_empty() {
                break;
            }

            let _: syn::Token![,] = content.parse()?;
        }

        Ok(Self {
            ty,
            fields: Some(fields),
        })
    }
}

pub struct Args {
    pub name: Ident,
    pub lang: Ident,
    pub message: MessageField,
    pub supplier: syn::Expr,
    pub file_type: ArgFileType,
    pub path: ArgPath,
}

pub enum ArgFileType {
    Toml,
    Json,
}

impl syn::parse::Parse for ArgFileType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit: LitStr = input.parse()?;
        match lit.value().as_str() {
            "toml" => Ok(Self::Toml),
            "json" => Ok(Self::Json),
            _ => Err(syn::Error::new(lit.span(), "expected toml or json")),
        }
    }
}

pub enum ArgPath {
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
            syn::custom_keyword!(file_type);
            syn::custom_keyword!(lang_file);
            syn::custom_keyword!(lang_folder);
        }

        macro_rules! parse {
            ($ident:ident) => {
                parse!($ident, Ident);
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
        parse!(message, MessageField, without_comma);

        if message.fields.is_some() {
            let _: syn::Token![,] = input.parse()?;
        }

        parse!(supplier, syn::Expr);

        parse!(file_type, ArgFileType);

        let crate_root = {
            #[allow(clippy::panic)]
            let crate_root = std::env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| panic!("failed to get CARGO_MANIFEST_DIR"));
            PathBuf::from(crate_root)
        };

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
            file_type,
            path,
        })
    }
}
