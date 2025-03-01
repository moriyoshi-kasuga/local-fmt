use enum_table::{EnumTable, Enumable};

pub mod message;
pub use message::*;

mod macros;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "derive")]
pub use local_fmt_derive::def_local_fmt;

pub enum LangSupplier<L> {
    Static(L),
    Dynamic(fn() -> L),
}

pub struct LocalFmt<L: Enumable, M, const N: usize> {
    messages: EnumTable<L, M, N>,
    lang: LangSupplier<L>,
}

impl<L: Enumable, M, const N: usize> LocalFmt<L, M, N> {
    pub const fn new(messages: EnumTable<L, M, N>, lang: LangSupplier<L>) -> Self {
        Self { messages, lang }
    }

    pub fn get_message(&self) -> &M {
        match &self.lang {
            LangSupplier::Static(lang) => self.messages.get(lang),
            LangSupplier::Dynamic(f) => {
                let lang = f();
                self.messages.get(&lang)
            }
        }
    }
}

impl<L: Enumable, M, const N: usize> std::ops::Deref for LocalFmt<L, M, N> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        self.get_message()
    }
}
