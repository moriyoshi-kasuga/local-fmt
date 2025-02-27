pub mod message;
use enum_table::{EnumTable, Enumable};
pub use message::*;

pub mod utils;
pub use utils::*;

use std::hash::Hash;

pub struct LocalFmt<L: Enumable, M, const N: usize> {
    messages: EnumTable<L, M, N>,
    global: fn() -> L,
}

impl<L: Eq + Hash + Enumable, M, const N: usize> LocalFmt<L, M, N> {
    pub fn new(messages: EnumTable<L, M, N>, global: fn() -> L) -> Self {
        Self { messages, global }
    }

    pub fn get_message(&self) -> &M {
        self.messages.get(&(self.global)())
    }
}
