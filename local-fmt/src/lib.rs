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

pub(crate) const fn u128_to_str(n: u128) -> ([u8; 39], usize) {
    if n == 0 {
        let buf: [u8; 39] = [
            b'0', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        return (buf, 1);
    }
    let mut buffer = [0u8; 39];
    let mut i = 0;
    let mut n = n;
    while n > 0 {
        buffer[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    let mut result = [0u8; 39];
    let mut j = 0;
    while j < i {
        result[j] = buffer[i - j - 1];
        j += 1;
    }
    (result, i)
}

#[allow(dead_code)]
pub(crate) const fn i128_to_str(n: i128) -> ([u8; 40], usize) {
    let (buf, total) = u128_to_str(n.unsigned_abs());
    let mut buffer = [0u8; 40];
    let mut i = 0;
    if n < 0 {
        buffer[i] = b'-';
        i += 1;
    }
    let mut j = 0;
    while j < total {
        buffer[i] = buf[j];
        i += 1;
        j += 1;
    }
    (buffer, i)
}

mod dev_macros {
    macro_rules! panic_builder {
        ($([$($message:tt)+]),* $(,)?) => {
            {
                let (buffer, total) = $crate::fmt_builder!(
                    $(
                        [$($message)+],
                    )*
                );
                let buffer = buffer.split_at(total).0;
                let message = unsafe { std::str::from_utf8_unchecked(buffer) };
                panic!("{}", message);
            }
        };
    }

    macro_rules! fmt_builder {
        (@ $var:ident, $message:literal) => {
            let $var = $message.as_bytes();
        };
        (@ $var:ident, $message:ident) => {
            let $var = $message.as_bytes();
        };
        (@ $var:ident, u = $message:expr ) => {
            let $var = $crate::u128_to_str($message as u128);
            let $var = $var.0.split_at($var.1).0;
        };
        (@ $var:ident, i = $message:expr) => {
            let $var = $crate::i128_to_str($message as i128);
            let $var = $var.0.split_at($var.1).0;
        };
        ($([$($message:tt)+]),* $(,)?) => {
            {
                let mut buffer = [0u8; 1024];
                let mut total = 0;
                $({
                    let mut i = 0;
                    $crate::fmt_builder!(@ message, $($message)+);
                    while i < message.len() {
                        buffer[total] = message[i];
                        i += 1;
                        total += 1;
                    }
                })*
                (buffer, total)
            }
        };
    }

    pub(crate) use fmt_builder;
    pub(crate) use panic_builder;
}

pub(crate) use dev_macros::*;
