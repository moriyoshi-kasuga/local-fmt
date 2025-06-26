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
    /// Creates a new LocalFmt instance with the given messages and language supplier function.
    /// 
    /// # Arguments
    /// * `messages` - An EnumTable containing messages for each language variant
    /// * `lang` - A function that returns the current language when called
    /// 
    /// # Example
    /// ```
    /// use local_fmt::LocalFmt;
    /// use enum_table::{EnumTable, et};
    /// 
    /// #[derive(Clone, Copy, enum_table::Enumable)]
    /// enum Lang { EN, JA }
    /// 
    /// struct Messages { hello: &'static str }
    /// 
    /// let messages = et!(Lang, Messages, |lang| match lang {
    ///     Lang::EN => Messages { hello: "Hello" },
    ///     Lang::JA => Messages { hello: "こんにちは" },
    /// });
    /// 
    /// let fmt = LocalFmt::new(messages, || Lang::EN);
    /// assert_eq!(fmt.get_message().hello, "Hello");
    /// ```
    pub const fn new(messages: EnumTable<L, M, N>, lang: fn() -> L) -> Self {
        Self { messages, lang }
    }

    /// Returns the message in the current language.
    /// 
    /// This method calls the language supplier function to determine the current language,
    /// then returns the corresponding message from the internal EnumTable.
    /// 
    /// # Example
    /// ```
    /// use local_fmt::LocalFmt;
    /// use enum_table::{EnumTable, et};
    /// use std::sync::RwLock;
    /// 
    /// #[derive(Clone, Copy, enum_table::Enumable)]
    /// enum Lang { EN, JA }
    /// 
    /// struct Messages { greeting: &'static str }
    /// 
    /// static CURRENT_LANG: RwLock<Lang> = RwLock::new(Lang::EN);
    /// 
    /// let messages = et!(Lang, Messages, |lang| match lang {
    ///     Lang::EN => Messages { greeting: "Hello" },
    ///     Lang::JA => Messages { greeting: "こんにちは" },
    /// });
    /// 
    /// let fmt = LocalFmt::new(messages, || *CURRENT_LANG.read().unwrap());
    /// 
    /// assert_eq!(fmt.get_message().greeting, "Hello");
    /// 
    /// *CURRENT_LANG.write().unwrap() = Lang::JA;
    /// assert_eq!(fmt.get_message().greeting, "こんにちは");
    /// ```
    pub fn get_message(&self) -> &M {
        self.messages.get(&(self.lang)())
    }

    /// Returns the current language by calling the language supplier function.
    /// 
    /// # Example
    /// ```
    /// use local_fmt::LocalFmt;
    /// use enum_table::{EnumTable, et};
    /// 
    /// #[derive(Clone, Copy, Debug, PartialEq, enum_table::Enumable)]
    /// enum Lang { EN, JA }
    /// 
    /// struct Messages { hello: &'static str }
    /// 
    /// let messages = et!(Lang, Messages, |lang| match lang {
    ///     Lang::EN => Messages { hello: "Hello" },
    ///     Lang::JA => Messages { hello: "こんにちは" },
    /// });
    /// 
    /// let fmt = LocalFmt::new(messages, || Lang::EN);
    /// assert_eq!(fmt.lang(), Lang::EN);
    /// ```
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
