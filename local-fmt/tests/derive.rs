#![cfg(feature = "macros")]

use std::sync::RwLock;

use enum_table::Enumable;
use local_fmt::def_local_fmt;
use local_fmt::ConstMessage;

#[derive(Clone, Copy, Enumable)]
enum Lang {
    EN,
    JA,
}

struct InnerMessages {
    pub hey: ConstMessage<0>,
}

struct Messages {
    pub inner: InnerMessages,
    pub hello: ConstMessage<1>,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::EN);

#[allow(clippy::unwrap_used)]
fn get_lang() -> Lang {
    *LANG.read().unwrap()
}

def_local_fmt!(
    name = MESSAGES,
    lang = Lang,
    message = Messages {
        inner: InnerMessages,
    },
    dynamic_supplier = get_lang,
    lang_file = "tests/lang.toml"
);

#[test]
fn normal() {
    assert_eq!(MESSAGES.hello.format(&["Rust"]), "Hello, world! Rust");

    *LANG.write().unwrap() = Lang::JA;

    assert_eq!(MESSAGES.hello.format(&["Rust"]), "こんにちは、世界！ Rust");
}
