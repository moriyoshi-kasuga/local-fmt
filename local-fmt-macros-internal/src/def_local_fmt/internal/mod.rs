use arg::{LangMessage, Message, MessageValue};

use crate::utils::hierarchy::Hierarchy;

use super::arg::{ArgFileType, ArgPath, MessageField};

mod arg;
mod file;

pub(crate) fn generate(
    file_type: ArgFileType,
    path: ArgPath,
    message: &MessageField,
) -> Vec<LangMessage> {
    let lang_messages = file::parse(file_type, path);

    for lang_message in &lang_messages {
        check_lang_message(
            &lang_message.lang,
            &lang_message.messages,
            &mut Hierarchy::new(),
            message,
        );
    }

    lang_messages
}

fn check_lang_message(
    lang: &str,
    messages: &Vec<Message>,
    hierarchy: &mut Hierarchy<String>,
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
                        hierarchy.process(ty.to_string(), |hierarchy| {
                            check_lang_message(lang, nested, hierarchy, field);
                        });
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
