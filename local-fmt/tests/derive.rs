#![cfg(feature = "macros")]
#![cfg(feature = "macros-toml")]

use std::sync::RwLock;

use enum_table::Enumable;
use local_fmt::{def_local_fmt, ConstMessage};

#[derive(Clone, Copy, Enumable)]
enum Lang {
    EN,
    JA,
}

struct Inner {
    pub name: &'static str,
}

struct Messages {
    pub inner: Inner,
    pub hello: ConstMessage<1>,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::EN);

def_local_fmt!(
    name = MESSAGES,
    lang = Lang,
    message = Messages { inner: Inner },
    supplier = || *LANG.read().unwrap(),
    file_type = "toml",
    lang_file = "tests/lang.toml"
);

#[test]
fn normal() {
    assert_eq!(MESSAGES.hello.format(&["Rust"]), "Hello, world! Rust");
    assert_eq!(MESSAGES.inner.name, "world");

    *LANG.write().unwrap() = Lang::JA;

    assert_eq!(MESSAGES.hello.format(&["Rust"]), "こんにちは、世界！ Rust");
    assert_eq!(MESSAGES.inner.name, "世界");
}
