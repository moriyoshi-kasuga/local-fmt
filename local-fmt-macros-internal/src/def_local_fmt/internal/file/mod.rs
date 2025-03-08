use std::error::Error;

use crate::def_local_fmt::arg::ArgFileType;

use super::{arg::MessageValue, ArgPath, LangMessage};

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "toml")]
mod toml;

pub(super) fn parse(file_type: ArgFileType, path: ArgPath) -> Vec<LangMessage> {
    match file_type {
        ArgFileType::Toml => {
            #[cfg(feature = "toml")]
            {
                toml::TomlMessageLoader::from_path(path)
            }
            #[cfg(not(feature = "toml"))]
            {
                panic!("toml feature is not enabled")
            }
        }
        ArgFileType::Json => {
            #[cfg(feature = "json")]
            {
                json::JsonMessageLoader::from_path(path)
            }
            #[cfg(not(feature = "json"))]
            {
                panic!("json feature is not enabled")
            }
        }
    }
}

pub(super) trait MessageLoader: Sized {
    const EXTENSION: &'static str;

    type Value;
    type NestValue;
    const NEST_VALUE_NAME: &'static str;

    fn value_to_nest(value: Self::Value) -> Option<Self::NestValue>;
    fn value_as_str(value: &Self::Value) -> Option<&str>;
    fn value_from_str(content: &str) -> Result<Self::Value, impl Error>;
    fn iter_nested(value: Self::NestValue) -> impl Iterator<Item = (String, Self::Value)>;

    fn from_path(path: ArgPath) -> Vec<LangMessage> {
        match path {
            ArgPath::File(file) => Self::from_file(file),
            ArgPath::Folder(folder) => Self::from_folder(folder),
        }
    }

    fn from_file(file: std::path::PathBuf) -> Vec<LangMessage> {
        let extension = file.extension().unwrap_or_else(|| {
            panic!(
                "Failed to retrieve file extension for path: {}",
                file.display()
            )
        });
        if extension != Self::EXTENSION {
            panic!(
                "Expected a {} file, but got {} file: {}",
                Self::EXTENSION,
                extension.to_string_lossy(),
                file.display()
            )
        }

        let content = std::fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("failed to read {}", file.display()));

        let value = Self::value_from_str(&content).unwrap_or_else(|e| {
            panic!(
                "failed to parse {} in {}: {}",
                Self::EXTENSION,
                file.display(),
                e
            )
        });

        let nest = Self::value_to_nest(value).unwrap_or_else(|| {
            panic!(
                "Expected a {} in the {} file: {}",
                Self::NEST_VALUE_NAME,
                Self::EXTENSION,
                file.display()
            )
        });

        let mut lang_messages = Vec::new();
        for (lang, value) in Self::iter_nested(nest) {
            let nest = Self::value_to_nest(value).unwrap_or_else(|| {
                panic!(
                    "Expected a {} in the {} file: {}",
                    Self::NEST_VALUE_NAME,
                    Self::EXTENSION,
                    file.display()
                )
            });
            let messages = Self::internal(&lang, &mut Vec::new(), nest);
            lang_messages.push(LangMessage { lang, messages });
        }
        lang_messages
    }

    fn from_folder(folder: std::path::PathBuf) -> Vec<LangMessage> {
        let files = folder.read_dir().unwrap_or_else(|_| {
            panic!(
                "Failed to read directory entry in folder: {}",
                folder.display()
            )
        });
        let mut lang_messages = Vec::new();
        for entry in files {
            let entry =
                entry.unwrap_or_else(|_| panic!("failed to read entry in {}", folder.display()));
            let path = entry.path();
            let exn = path.extension().unwrap_or_else(|| {
                panic!(
                    "Failed to retrieve file extension for path: {}",
                    path.display()
                )
            });
            if exn != Self::EXTENSION {
                continue;
            }
            let lang = path
                .file_stem()
                .unwrap_or_else(|| panic!("failed to get file stem in {}", path.display()))
                .to_string_lossy();

            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
            let value = Self::value_from_str(&content).unwrap_or_else(|e| {
                panic!(
                    "failed to parse {} in {}: {}",
                    Self::EXTENSION,
                    path.display(),
                    e
                )
            });

            let nest = Self::value_to_nest(value).unwrap_or_else(|| {
                panic!(
                    "Expected a {} in the {} file: {}",
                    Self::NEST_VALUE_NAME,
                    Self::EXTENSION,
                    path.display()
                )
            });

            let messages = Self::internal(&lang, &mut Vec::new(), nest);
            lang_messages.push(LangMessage {
                lang: lang.to_string(),
                messages,
            });
        }
        lang_messages
    }

    fn internal(
        lang: &str,
        hierarchy: &mut Vec<String>,
        value: Self::NestValue,
    ) -> Vec<super::Message> {
        let mut messages = Vec::new();
        for (key, value) in Self::iter_nested(value) {
            if let Some(value) = Self::value_as_str(&value) {
                messages.push(super::Message {
                    value: MessageValue::Token(value.parse().unwrap_or_else(|e| {
                        let mut display_key = if value.is_empty() {
                            hierarchy.join(".")
                        } else {
                            String::new()
                        };
                        display_key.push_str(&key);
                        panic!(
                            "Failed to parse message token for language '{}' and key '{}': {}",
                            lang, display_key, e
                        )
                    })),
                    key,
                });
                continue;
            }
            let nest = Self::value_to_nest(value).unwrap_or_else(|| {
                let mut display_key = if key.is_empty() {
                    hierarchy.join(".")
                } else {
                    String::new()
                };
                display_key.push_str(&key);
                panic!(
                    "Expected a string or {} for language '{}' and key '{}'",
                    Self::NEST_VALUE_NAME,
                    lang,
                    display_key
                )
            });
            let temp_key = key.clone();
            hierarchy.push(temp_key);
            let nest_messages = Self::internal(lang, hierarchy, nest);
            hierarchy.pop();
            messages.push(super::Message {
                key,
                value: MessageValue::Nested(nest_messages),
            });
        }
        messages
    }
}
