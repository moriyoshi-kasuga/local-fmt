#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use enum_table::{EnumTable, Enumable};

pub mod message;
pub use message::*;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "macros")]
#[doc(inline)]
pub use local_fmt_macros::{def_local_fmt, gen_const_message, gen_message};
#[cfg(feature = "macros")]
pub mod macros;

pub struct LocalFmt<L: Enumable + Copy, M, const N: usize> {
    messages: EnumTable<L, M, N>,
    lang: fn() -> L,
}

impl<L: Enumable + Copy, M, const N: usize> LocalFmt<L, M, N> {
    pub const fn new(messages: EnumTable<L, M, N>, lang: fn() -> L) -> Self {
        Self { messages, lang }
    }

    pub fn get_message(&self) -> &M {
        self.messages.get(&(self.lang)())
    }

    pub fn lang(&self) -> L {
        (self.lang)()
    }
}

impl<L: Enumable + Copy, M, const N: usize> std::ops::Deref for LocalFmt<L, M, N> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        self.get_message()
    }
}

macro_rules! panic_builder {
        ($($message:expr),* $(,)?) => {
            {
                let mut buffer = [0u8; 1024];
                let mut i = 0;
                $(
                    let message = $message;
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

pub(crate) use panic_builder;
