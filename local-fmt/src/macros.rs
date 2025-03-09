use std::marker::PhantomData;

use crate::ConstMessage;

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
            dev_macros::panic_builder!(
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

impl<const N: usize> CheckConstMessageArg<ConstMessage<N>, &'static str> {
    pub const fn check(lang: &'static str, key: &'static str, _: ConstMessage<N>) -> &'static str {
        dev_macros::panic_builder!(
            "Error: A message with no arguments was expected, but received a message with "
                .as_bytes(),
            N.to_ne_bytes(),
            " arguments. This occurred in the language '".as_bytes(),
            lang.as_bytes(),
            "' with the key '".as_bytes(),
            key.as_bytes(),
            "'. Please check the message definition and ensure the correct number of arguments."
                .as_bytes(),
        )
    }
}

impl CheckConstMessageArg<&'static str, &'static str> {
    pub const fn check(
        _lang: &'static str,
        _key: &'static str,
        text: &'static str,
    ) -> &'static str {
        text
    }
}

impl<const M: usize> CheckConstMessageArg<&'static str, ConstMessage<M>> {
    pub const fn check(lang: &'static str, key: &'static str, _: &'static str) -> ConstMessage<M> {
        dev_macros::panic_builder!(
            "Error: A message with ".as_bytes(),
            M.to_ne_bytes(),
            " arguments was expected, but received a message with no arguments. This occurred in the language '".as_bytes(),
            lang.as_bytes(),
            "' with the key '".as_bytes(),
            key.as_bytes(),
            "'. Please check the message definition and ensure the correct number of arguments.".as_bytes(),
        )
    }
}

mod dev_macros {
    macro_rules! panic_builder {
        ($($message:expr),* $(,)?) => {
            {
                let mut buffer = [0u8; 1024];
                $(
                    let message = $message;
                    let mut i = 0;
                    while i < message.len() {
                        buffer[i] = message[i];
                        i += 1;
                    }
                )*
                let message = unsafe { std::str::from_utf8_unchecked(&buffer) };
                panic!("{}", message);
            }
        };
    }

    pub(super) use panic_builder;
}
