# [LocalFmt][docsrs]: Provide simple localizable format strings

[![LocalFmt on crates.io][cratesio-image]][cratesio]
[![LocalFmt on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/local-fmt.svg
[cratesio]: https://crates.io/crates/local-fmt
[docsrs-image]: https://docs.rs/local-fmt/badge.svg
[docsrs]: https://docs.rs/local-fmt

## Overview

`local-fmt` is a Rust library designed to simplify the process of creating applications that support multiple languages. It allows developers to define localizable format strings in a structured and maintainable way, using familiar file formats like TOML, JSON, and YAML.

## Crate Configuration Features

To customize the functionality of `local-fmt`, you can enable the following features in your `Cargo.toml`:

- **macros**: Includes macros such as `gen_static_message`, `gen_alloc_message`, and `def_local_fmt`.
  If you use `def_local_fmt`, choose one of the following:
  - **macros-toml**: Enables parsing of TOML files.
  - **macros-json**: Enables parsing of JSON files.
  - **macros-yaml**: Enables parsing of YAML files.

## Install

```toml
[dependencies]
enum-table = "0.2" # Required if you use `def_local_fmt` macro or call the `new` function of LocalFmt
local_fmt = { version = "0.5", features = ["macros", "macros-toml"] }
```

## Key Features

- **Localizable Messages**: Easily define messages in multiple languages using TOML, JSON, or YAML files.
- **Dynamic Language Switching**: Change the language at runtime using a function pointer, allowing for flexible language management.
- **Compile-time Checks**: Ensure the correctness of message formats at compile time by:
  - Verifying that the number of arguments matches the placeholders.
  - Ensuring that all required arguments are present.
  - Providing detailed error messages that specify which language key is affected, helping you quickly identify and resolve issues.
- **Integration with Serde**: Optionally serialize and deserialize messages for persistent storage or network transmission.

## Usage Example

Here's a simple example demonstrating how to use `local-fmt` in a Rust project.
This example shows how to create a localizable format string and use it to format a message.

```rust
#![cfg(feature = "macros")]
#![cfg(feature = "macros-toml")]

use std::sync::RwLock;
use enum_table::Enumable;
use local_fmt::{def_local_fmt, StaticMessage};

#[derive(Clone, Copy, Enumable)]
enum Lang {
    EN,
    JA,
}

struct Words {
    // If there are no placeholders, use &'static str instead of StaticMessage<0>
    pub ownership: &'static str,
}

// Nested struct example
struct Messages {
    pub hello: StaticMessage<1>,
    pub words: Words,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::EN);

def_local_fmt!(
    name = MESSAGES,
    lang = Lang,
    message = Messages {
      words: Words,
    },
    supplier = || *LANG.read().unwrap(),
    file_type = "toml",
    lang_folder = "doctest/langs"
);

// Example content of doctest/lang/EN.toml

// hello = "Hello, world! {0}"
//
// [words]
// ownership = "ownership"

fn main() {
    // Use the `MESSAGES` to create a personalized greeting message.
    assert_eq!(MESSAGES.hello.format(&["Rust"]), "Hello, world! Rust");
    assert_eq!(MESSAGES.words.ownership, "ownership");

    // Change the language to Japanese
    *LANG.write().unwrap() = Lang::JA;

    // Print the greeting message in Japanese
    assert_eq!(MESSAGES.hello.format(&["Rust"]), "こんにちは、世界！ Rust");
    assert_eq!(MESSAGES.words.ownership, "所有権");
}
```

For more detailed information on `def_local_fmt`, see the [documentation](https://docs.rs/local-fmt/macro.def_local_fmt.html).

## License

Licensed under

- [MIT license](https://github.com/moriyoshi-kasuga/local-fmt/blob/main/LICENSE)
