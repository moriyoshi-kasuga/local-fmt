use arg::{LangMessage, Message, MessageValue};
use file::{json::JsonMessageLoader, toml::TomlMessageLoader, MessageLoader};

use super::arg::{ArgFileType, ArgPath, MessageField};

mod arg;
mod file;

pub(crate) fn generate(
    file_type: ArgFileType,
    path: ArgPath,
    message: &MessageField,
) -> Vec<LangMessage> {
    let lang_messages = match file_type {
        ArgFileType::Toml => TomlMessageLoader::from_path(path),
        ArgFileType::Json => JsonMessageLoader::from_path(path),
    };

    for lang_message in &lang_messages {
        check_lang_message(
            &lang_message.lang,
            &lang_message.messages,
            &mut Vec::new(),
            message,
        );
    }

    lang_messages
}

fn check_lang_message(
    lang: &str,
    messages: &Vec<Message>,
    hierarchy: &mut Vec<String>,
    field: &MessageField,
) {
    match &field.fields {
        None => {
            for message in messages {
                if let MessageValue::Nested(_) = &message.value {
                    panic!(
                        "Expected a simple message, but got a nested message in language {}: {}",
                        lang, message.key
                    );
                }
            }
        }
        Some(fields) => {
            for (ty, field) in fields {
                let message = messages.iter().find(|m| *ty == m.key).unwrap_or_else(|| {
                    panic!(
                        "Expected a nest message with key {}, but got nothing in language {}",
                        ty, lang
                    )
                });

                match &message.value {
                    MessageValue::Nested(nested) => {
                        hierarchy.push(message.key.clone());
                        check_lang_message(lang, nested, hierarchy, field);
                        hierarchy.pop();
                    }
                    _ => {
                        panic!(
                            "Expected a nested message, but got a simple message in language {}: {}",
                            lang, message.key
                        );
                    }
                }
            }
        }
    }
}
