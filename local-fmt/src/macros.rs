use std::marker::PhantomData;

use local_fmt_macros::gen_static_message;

use crate::{panic_builder, StaticMessage};

pub struct CheckStaticMessageArg<From, To>(PhantomData<(From, To)>);

impl<From, To> CheckStaticMessageArg<From, To> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<const M: usize, const N: usize> CheckStaticMessageArg<StaticMessage<N>, StaticMessage<M>> {
    #[track_caller]
    pub const fn check(
        lang: &'static str,
        key: &'static str,
        arg: StaticMessage<N>,
    ) -> StaticMessage<M> {
        if N == M {
            unsafe { std::mem::transmute::<StaticMessage<N>, StaticMessage<M>>(arg) }
        } else {
            const MESSAGE: StaticMessage<4> = gen_static_message!("Error: A message with {0} arguments was expected, but received a message with {1} arguments. This occurred in the language '{2}' with the key '{3}'. Please check the message definition and ensure the correct number of arguments.");
            panic_builder!(MESSAGE, [u = M], [u = N], [lang], [key],)
        }
    }
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
