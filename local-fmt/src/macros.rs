use std::marker::PhantomData;

use crate::{panic_builder, ConstMessage};

pub struct CheckConstMessageArg<From, To>(PhantomData<(From, To)>);

impl<From, To> CheckConstMessageArg<From, To> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<const M: usize, const N: usize> CheckConstMessageArg<ConstMessage<N>, ConstMessage<M>> {
    pub const fn check(
        lang: &'static str,
        key: &'static str,
        arg: ConstMessage<N>,
    ) -> ConstMessage<M> {
        if N == M {
            unsafe { std::mem::transmute::<ConstMessage<N>, ConstMessage<M>>(arg) }
        } else {
            panic_builder!(
                "Error: A message with ".as_bytes(),
                M.to_ne_bytes(),
                " arguments was expected, but received a message with ".as_bytes(),
                N.to_ne_bytes(),
                " arguments. This occurred in the language '".as_bytes(),
                lang.as_bytes(),
                "' with the key '".as_bytes(),
                key.as_bytes(),
                "'. Please check the message definition and ensure the correct number of arguments.".as_bytes(),
            )
        }
    }
}

// impl<const N: usize> CheckConstMessageArg<ConstMessage<N>, &'static str> {
//     pub const fn check(lang: &'static str, key: &'static str, _: ConstMessage<N>) -> &'static str {
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

// impl CheckConstMessageArg<&'static str, &'static str> {
//     pub const fn check(
//         _lang: &'static str,
//         _key: &'static str,
//         text: &'static str,
//     ) -> &'static str {
//         text
//     }
// }

// impl<const M: usize> CheckConstMessageArg<&'static str, ConstMessage<M>> {
//     pub const fn check(lang: &'static str, key: &'static str, _: &'static str) -> ConstMessage<M> {
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
