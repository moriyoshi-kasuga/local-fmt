use local_fmt_macros::gen_static_message;

use crate::{panic_builder, StaticMessage};

pub trait CheckStaticMessageArg<To: Sized>: Sized {
    const IS_INVALID: Option<StaticMessage<2>>;
}

#[track_caller]
pub const fn check_static_message_arg<To, From>(
    lang: &'static str,
    key: &'static str,
    from: &From,
) -> To
where
    From: CheckStaticMessageArg<To>,
{
    if let Some(message) = From::IS_INVALID {
        panic_builder!(message, [lang], [key]);
    }
    unsafe { std::ptr::read(from as *const From as *const To) }
}

impl CheckStaticMessageArg<&str> for &str {
    const IS_INVALID: Option<StaticMessage<2>> = None;
}

impl<const N: usize, const M: usize> CheckStaticMessageArg<StaticMessage<N>> for StaticMessage<M> {
    const IS_INVALID: Option<StaticMessage<2>> = if N == M {
        None
    } else {
        Some(gen_static_message!(
            "Error: A message with {u:M} arguments was expected in the language '{0}', ",
            "but received a message with {u:N} arguments for the key '{1}'. ",
            "Please check the message definition and ensure the correct number of arguments."
        ))
    };
}

impl<const N: usize> CheckStaticMessageArg<&str> for StaticMessage<N> {
    const IS_INVALID: Option<StaticMessage<2>> = Some(gen_static_message!(
        "Error: A message with {u:N} arguments was expected in the language '{0}', ",
        "but received a message with no arguments for the key '{1}'. ",
        "Please check the message definition and ensure the correct number of arguments."
    ));
}

impl<const N: usize> CheckStaticMessageArg<StaticMessage<N>> for &str {
    const IS_INVALID: Option<StaticMessage<2>> = Some(gen_static_message!(
        "Error: A message with {u:N} arguments was expected in the language '{0}', ",
        "but received a message with no arguments for the key '{1}'. ",
        "Please check the message definition and ensure the correct number of arguments."
    ));
}
