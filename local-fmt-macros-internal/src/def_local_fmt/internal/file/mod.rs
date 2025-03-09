use crate::{def_local_fmt::arg::ArgFileType, utils::hierarchy::Hierarchy};

use super::{arg::MessageValue, ArgPath, LangMessage};

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "yaml")]
mod yaml;

pub(super) fn parse(file_type: ArgFileType, path: ArgPath) -> Vec<LangMessage> {
    macro_rules! from_path {
        ($file_type:ident, $path:ident, {$($pattern:pat => ($feature:literal, $mod:ident::$loader:ident),)+}) => {
            use ArgFileType::*;
            match $file_type {
                $(
                    $pattern => {
                        #[cfg(feature = $feature)]
                        {
                            $mod::$loader::from_path($path)
                        }
                        #[cfg(not(feature = $feature))]
                        {
                            panic!(concat!($feature, " feature is not enabled"))
                        }
                    },
                )+
            }
        };
    }

    from_path! { file_type, path, {
        Toml => ("toml", toml::TomlMessageLoader),
        Json => ("json", json::JsonMessageLoader),
        Yaml => ("yaml", yaml::YamlMessageLoader),
    } }
}

pub(super) trait MessageLoader: Sized {
    const EXTENSION: &'static str;

    type Value;
    type NestValue;
    const NEST_VALUE_NAME: &'static str;

    fn value_to_nest(value: Self::Value) -> Option<Self::NestValue>;
    fn value_as_str(value: &Self::Value) -> Option<&str>;
    fn value_from_str(content: &str) -> Result<Self::Value, String>;
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
            let messages = Self::internal(&lang, &mut Hierarchy::new(), nest);
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

            let messages = Self::internal(&lang, &mut Hierarchy::new(), nest);
            lang_messages.push(LangMessage {
                lang: lang.to_string(),
                messages,
            });
        }
        lang_messages
    }

    fn internal(
        lang: &str,
        hierarchy: &mut Hierarchy<String>,
        value: Self::NestValue,
    ) -> Vec<super::Message> {
        let mut messages = Vec::new();
        for (key, value) in Self::iter_nested(value) {
            if let Some(value) = Self::value_as_str(&value) {
                messages.push(super::Message {
                    value: MessageValue::Token(value.parse().unwrap_or_else(|e| {
                        let key = hierarchy.join(value);
                        panic!(
                            "Failed to parse message token for language '{}' and key '{}': {}",
                            lang, key, e
                        )
                    })),
                    key,
                });
                continue;
            }
            let nest = Self::value_to_nest(value).unwrap_or_else(|| {
                let display_key = hierarchy.join(&key);
                panic!(
                    "Expected a string or {} for language '{}' and key '{}'",
                    Self::NEST_VALUE_NAME,
                    lang,
                    display_key
                )
            });
            let temp_key = key.clone();
            let nest_messages =
                hierarchy.process(temp_key, |hierarchy| Self::internal(lang, hierarchy, nest));
            messages.push(super::Message {
                key,
                value: MessageValue::Nested(nest_messages),
            });
        }
        messages
    }
}
