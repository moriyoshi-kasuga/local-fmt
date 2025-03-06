use arg::{LangMessage, Message};
use file::{json::JsonMessageLoader, toml::TomlMessageLoader, MessageLoader};
use proc_macro2::TokenStream;

use super::arg::{ArgFileType, ArgPath, MessageField};

mod arg;
mod file;

pub(crate) fn generate(
    file_type: ArgFileType,
    path: ArgPath,
    message: &MessageField,
) -> TokenStream {
    let lang_messages = match file_type {
        ArgFileType::Toml => TomlMessageLoader::from_path(path),
        ArgFileType::Json => JsonMessageLoader::from_path(path),
    };

    todo!();
}
