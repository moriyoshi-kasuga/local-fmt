use local_fmt_macros::gen_static_message;

use crate::{panic_builder, StaticMessage};

pub trait CheckStaticMessageArg<To> {
    const IS_VALID: Option<StaticMessage<2>>;
}

pub const fn check_static_message_arg<To, From>(
    lang: &'static str,
    key: &'static str,
    from: &From,
) -> Option<StaticMessage<2>>
where
    From: CheckStaticMessageArg<To>,
{
    if let Some(message) = From::IS_VALID {
        panic_builder!(message, [lang], [key])
    }
    unsafe { std::mem::transmute_copy(&from) }
}

impl CheckStaticMessageArg<&'static str> for &'static str {
    const IS_VALID: Option<StaticMessage<2>> = None;
}

impl<const N: usize, const M: usize> CheckStaticMessageArg<StaticMessage<N>> for StaticMessage<M> {
    const IS_VALID: Option<StaticMessage<2>> = if N == M {
        None
    } else {
        Some(gen_static_message!("Error: A message with {0} arguments was expected, but received a message with {1} arguments. Please check the message definition and ensure the correct number of arguments."))
    };
}

impl<const N: usize> CheckStaticMessageArg<&'static str> for StaticMessage<N> {
    const IS_VALID: Option<StaticMessage<2>> = Some(gen_static_message!("Error: A message with no arguments was expected in the language '{0}', but received a message with arguments for the key '{1}'. Please check the message definition and ensure the correct number of arguments."));
}

impl<const N: usize> CheckStaticMessageArg<StaticMessage<N>> for &'static str {
    const IS_VALID: Option<StaticMessage<2>> = Some(gen_static_message!("Error: A message with {0} arguments was expected in the language '{1}', but received a message with no arguments. Please check the message definition and ensure the correct number of arguments."));
}

// impl<const N: usize> CheckStaticMessageArg<StaticMessage<N>, &'static str> {
//     pub const fn check(lang: &'static str, key: &'static str, _: StaticMessage<N>) -> &'static str {
//         dev_macros::panic_builder!(
//             "Error: A message with no arguments was expected, but received a message with "
//                 .as_bytes(),
//             N.to_ne_bytes(),
//             " arguments. This occurred in the language '".as_bytes(),
//             lang.as_bytes(),
//             "' with the key '".as_bytes(),
//             key.as_bytes(),
//             "'. Please check the message definition and ensure the correct number of arguments."
//                 .as_bytes(),
//         )
//     }
// }

// impl CheckStaticMessageArg<&'static str, &'static str> {
//     pub const fn check(
//         _lang: &'static str,
//         _key: &'static str,
//         text: &'static str,
//     ) -> &'static str {
//         text
//     }
// }

// impl<const M: usize> CheckStaticMessageArg<&'static str, StaticMessage<M>> {
//     pub const fn check(lang: &'static str, key: &'static str, _: &'static str) -> StaticMessage<M> {
//         dev_macros::panic_builder!(
//             "Error: A message with ".as_bytes(),
//             M.to_ne_bytes(),
//             " arguments was expected, but received a message with no arguments. This occurred in the language '".as_bytes(),
//             lang.as_bytes(),
//             "' with the key '".as_bytes(),
//             key.as_bytes(),
//             "'. Please check the message definition and ensure the correct number of arguments.".as_bytes(),
//         )
//     }
// }
