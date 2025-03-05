use std::{path::Path, str::FromStr};

use arg::{LangMessage, Message};

use crate::parse::{MessageToken, MessageTokenValue};

use super::arg::ArgPath;

mod arg;

pub(crate) fn generate(path: ArgPath) -> Vec<arg::LangMessage> {
    match path {
        ArgPath::File(file) => {
            let content = std::fs::read_to_string(&file)
                .unwrap_or_else(|_| panic!("failed to read {}", file.display()));
            let toml: toml::Value = toml::from_str(&content)
                .unwrap_or_else(|_| panic!("failed to parse toml in {}", file.display()));
            let table = match toml {
                toml::Value::Table(table) => table,
                _ => panic!(
                    "Expected a table structure in the TOML file: {}",
                    file.display()
                ),
            };
            let mut lang_messages = Vec::new();
            for (lang, value) in table {
                let messages = internal(file.as_path(), &lang, value);
                lang_messages.push(LangMessage { lang, messages });
            }
            lang_messages
        }
        ArgPath::Folder(folder) => {
            let files = folder.read_dir().unwrap_or_else(|_| {
                panic!(
                    "Failed to read directory entry in folder: {}",
                    folder.display()
                )
            });
            let mut lang_messages = Vec::new();
            for entry in files {
                let entry = entry
                    .unwrap_or_else(|_| panic!("failed to read entry in {}", folder.display()));
                let path = entry.path();
                if path.extension().unwrap_or_else(|| {
                    panic!(
                        "Failed to retrieve file extension for path: {}",
                        path.display()
                    )
                }) != "toml"
                {
                    continue;
                }
                let lang = path
                    .file_stem()
                    .unwrap_or_else(|| panic!("failed to get file stem in {}", path.display()))
                    .to_string_lossy();

                let content = std::fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
                let toml: toml::Value = toml::from_str(&content)
                    .unwrap_or_else(|_| panic!("failed to parse toml in {}", path.display()));
                let messages = internal(path.as_path(), &lang, toml);
                lang_messages.push(LangMessage {
                    lang: lang.to_string(),
                    messages,
                });
            }
            lang_messages
        }
    }
}

fn internal(file_path: &Path, lang: &str, value: toml::Value) -> Vec<arg::Message> {
    let table = match value {
        toml::Value::Table(table) => table,
        _ => panic!("expected table in {} for {}", file_path.display(), lang),
    };
    let mut messages = Vec::new();
    for (name, value) in table {
        let text = match value {
            toml::Value::String(text) => text,
            _ => panic!(
                "expected string in {} for {} in {}",
                file_path.display(),
                name,
                lang
            ),
        };

        let message = MessageToken::from_str(&text).unwrap_or_else(|err| {
            panic!(
                "Failed to parse message token for language '{}' and key '{}': {}",
                lang, &name, err
            )
        });
        for value in &message.values {
            if let MessageTokenValue::PlaceholderIdent(ident) = value {
                panic!(
                    "Placeholder '{{{}}}' is not allowed in the message token for language '{}' and key '{}'",
                    ident, lang, &name
                );
            }
        }
        messages.push(Message {
            name,
            value: message,
        });
    }
    messages
}
