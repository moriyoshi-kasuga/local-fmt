use std::collections::HashMap;
use std::hash::Hash;

#[cfg(feature = "macros")]
pub use local_fmt_macros as macros;

pub mod message;
pub use message::*;

pub struct LocalFmt<L, M> {
    messages: HashMap<L, M>,
    global: fn() -> L,
}

impl<L: Eq + Hash, M> LocalFmt<L, M> {
    pub fn new(messages: HashMap<L, M>, global: fn() -> L) -> Self {
        Self { messages, global }
    }

    pub fn get_message(&self) -> &M {
        #[allow(clippy::expect_used)]
        self.messages
            .get(&(self.global)())
            .expect("message not found")
    }
}
