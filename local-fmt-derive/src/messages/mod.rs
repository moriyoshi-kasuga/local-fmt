#![allow(clippy::panic)]

use std::path::Path;

use arg::{LangMessage, MessageValue};

use crate::internal_def_local_fmt::ArgPath;

pub(crate) mod arg;

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

        let mut max_placeholder = None::<usize>;

        let mut value = Vec::new();

        let mut buffer = Vec::<u8>::new();

        let mut bytes = text.bytes();

        while let Some(byte) = bytes.next() {
            match byte {
                b'{' => {
                    value.push(MessageValue::Text(unsafe {
                        String::from_utf8_unchecked(std::mem::take(&mut buffer))
                    }));

                    let mut number = 0;

                    loop {
                        match bytes.next() {
                            Some(byte) => match byte {
                                b'}' => {
                                    match max_placeholder {
                                        Some(max) => {
                                            max_placeholder = Some(max.max(number));
                                        }
                                        None => {
                                            max_placeholder = Some(number);
                                        }
                                    }
                                    value.push(MessageValue::Placeholder(number));
                                    break;
                                }
                                b'0'..=b'9' => {
                                    number *= 10;
                                    number += (byte - b'0') as usize;
                                }
                                _ => {
                                    panic!("Missing closing brace for placeholder in message '{}' within file: {}", name, file_path.display())
                                }
                            },
                            None => {
                                panic!(
                                    "without number placeholder in {} for {}",
                                    file_path.display(),
                                    name
                                )
                            }
                        }
                    }
                }
                b'\\' => {
                    if let Some(byte) = bytes.next() {
                        match byte {
                            b'{' => buffer.push(b'{'),
                            _ => {
                                buffer.push(b'\\');
                                buffer.push(byte);
                            }
                        }
                    } else {
                        buffer.push(b'\\');
                    }
                }
                _ => buffer.push(byte),
            }
        }

        if let Some(max_placeholder) = max_placeholder {
            let mut numbers = vec![false; max_placeholder + 1];
            for v in &value {
                if let MessageValue::Placeholder(number) = v {
                    numbers[*number] = true;
                }
            }
            for (i, v) in numbers.iter().enumerate() {
                if !v {
                    panic!(
                        "Placeholder index {} is missing in message '{}' for language '{}'. The highest index found is {}.",
                        i, name, lang, max_placeholder
                    );
                }
            }
        }

        if !buffer.is_empty() {
            value.push(MessageValue::Text(unsafe {
                String::from_utf8_unchecked(buffer)
            }));
        }

        messages.push(arg::Message {
            name,
            values: value,
        });
    }
    messages
}
