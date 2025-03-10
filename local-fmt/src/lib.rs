#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::macro_metavars_in_unsafe)]

extern crate self as local_fmt;

use enum_table::{EnumTable, Enumable};

pub mod utils;
pub use utils::*;

pub mod message;
pub use message::*;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "macros")]
#[doc(inline)]
pub use local_fmt_macros::{def_local_fmt, gen_alloc_message, gen_static_message};
#[cfg(feature = "macros")]
pub mod macros;

/// A struct that holds a message and the language it is in.
pub struct LocalFmt<L: Enumable + Copy, M, const N: usize> {
    messages: EnumTable<L, M, N>,
    lang: fn() -> L,
}

impl<L: Enumable + Copy, M, const N: usize> LocalFmt<L, M, N> {
    pub const fn new(messages: EnumTable<L, M, N>, lang: fn() -> L) -> Self {
        Self { messages, lang }
    }

    /// Returns the message in the current language.
    pub fn get_message(&self) -> &M {
        self.messages.get(&(self.lang)())
    }

    /// Returns the language.
    pub fn lang(&self) -> L {
        (self.lang)()
    }
}

impl<L: Enumable + Copy, M, const N: usize> std::ops::Deref for LocalFmt<L, M, N> {
    type Target = M;

    /// Returns the message in the current language.
    fn deref(&self) -> &Self::Target {
        self.get_message()
    }
}
