# [LocalFmt][docsrs]: Provide simple localizable format strings

[![LocalFmt on crates.io][cratesio-image]][cratesio]
[![LocalFmt on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/local-fmt.svg
[cratesio]: https://crates.io/crates/local-fmt
[docsrs-image]: https://docs.rs/local-fmt/badge.svg
[docsrs]: https://docs.rs/local-fmt

## Overview

`local-fmt` is a Rust library that provides a mechanism for defining and using localizable format strings.
It is designed to facilitate the creation of applications that support multiple languages
by allowing developers to define messages in a structured and maintainable way.

## Features

- **Localizable Messages**: Define messages in multiple languages using TOML or JSON files.
- **Dynamic Language Switching**: Change the language at runtime using a dynamic supplier.
- **Compile-time Checks**: Ensure message format correctness at compile time.
- **Integration with Serde**: Optional support for serializing and deserializing messages.

## Usage

Below is an example of how to use `local-fmt` in a Rust project. This example demonstrates creating a localizable format string and using it to format a message.

```rust
# #![cfg(feature = "macros")]
# #![cfg(feature = "macros-toml")]
/// This example demonstrates how to use the `local-fmt` library to create
/// and use localizable format strings in a Rust application.

use std::sync::RwLock;
use enum_table::Enumable;
use local_fmt::{def_local_fmt, ConstMessage};

#[derive(Clone, Copy, Enumable)]
enum Lang {
    EN,
    JA,
}

struct Messages {
    pub hello: ConstMessage<1>,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::EN);

def_local_fmt!(
    name = MESSAGES,
    lang = Lang,
    message = Messages,
    supplier = || *LANG.read().unwrap(),
    file_type = "toml",
    lang_folder = "doctest/langs"
);

fn main() {
    // Use the `MESSAGES` to create a personalized greeting message.
    assert_eq!(MESSAGES.hello.format(&["Rust"]), "Hello, world! Rust");

    // Change the language to Japanese
    *LANG.write().unwrap() = Lang::JA;

    // Print the greeting message in Japanese
    assert_eq!(MESSAGES.hello.format(&["Rust"]), "こんにちは、世界！ Rust");
}
```

## Documentation

For more detailed information, please refer to the [documentation][docsrs].
