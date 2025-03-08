/// A procedural macro to define localized formatted messages.
///
/// This macro generates a set of constant messages that can be used for localization purposes.
/// It allows you to define messages in multiple languages and switch between them dynamically
/// based on a language supplier function.
///
/// # Arguments
///
/// * `name` - The name of the generated static message set.
/// * `lang` - The enumeration representing the supported languages.
/// * `message` - The struct containing the constant messages.
/// * `supplier` - The language supplier, type is fn() -> Lang. It determines how the current language is selected
/// * `def location` - Specifies the location of the language definition files. This can be either:
///     * `lang_file` - The path to a single language definition file.
///     * `lang_folder` - The folder containing multiple language definition files, one for each language.
///
/// # Notes
/// * The language definition file(s) must be in the TOML format.
/// * The `def location` expands to `CARGO_MANIFEST_DIR/{your_path}`, where `CARGO_MANIFEST_DIR`
///   is an environment variable representing the directory containing the Cargo.toml file of your project.
///   This ensures that paths are resolved relative to the project's root directory.
///
/// ## Example with `lang_file = "lang.toml"`
///
/// The language definition file should be structured as follows:
///
/// ```toml
/// # in lang.toml
///
/// # The table name corresponds to the language enumeration variant,
/// # and the message field matches the field in the message struct.
/// [EN]
/// hello = "Hello, world! {0}"
///
/// [JA]
/// hello = "こんにちは、世界！{0}"
/// ```
///
/// ## Example with `lang_folder = "langs"`
///
/// The folder should contain separate TOML files for each language:
///
/// <pre>
/// /langs
/// ├── EN.toml
/// └── JA.toml
/// </pre>
///
/// Each file should be formatted as follows:
///
/// ```toml
/// # in EN.toml
///
/// # The table name corresponds to the language enumeration variant.
/// hello = "Hello, world! {0}"
/// ```
/// ```toml
/// # in JA.toml
///
/// # The table name corresponds to the language enumeration variant.
/// hello = "こんにちは、世界！{0}"
/// ```
///
///
/// # Example 1
///
/// ```rust
/// # #![cfg(feature = "toml")]
///
/// use std::sync::RwLock;
/// use enum_table::Enumable;
/// use local_fmt::{def_local_fmt, ConstMessage};
///
/// #[derive(Clone, Copy, Enumable)]
/// enum Lang {
///     EN,
///     JA,
/// }
///
/// struct Messages {
///     pub hello: ConstMessage<1>,
/// }
///
/// static LANG: RwLock<Lang> = RwLock::new(Lang::EN);
///
/// def_local_fmt!(
///     name = MESSAGES,
///     lang = Lang,
///     message = Messages,
///     supplier = || *LANG.read().unwrap(),
///     file_type = "toml",
///     lang_folder = "doctest/langs"
/// );
///
/// assert_eq!(MESSAGES.hello.format(&["Rust"]), "Hello, world! Rust");
///
/// *LANG.write().unwrap() = Lang::JA;
///
/// assert_eq!(MESSAGES.hello.format(&["Rust"]), "こんにちは、世界！ Rust");
/// ```
///
/// # Example 2
/// ```
/// # #![cfg(feature = "json")]
///
/// use enum_table::Enumable;
/// use local_fmt::{def_local_fmt, ConstMessage, LocalFmt};
/// use std::sync::RwLock;
///
/// #[derive(Clone, Copy, Enumable)]
/// enum Lang {
///    EN,
///    JA,
/// }
///
/// struct ActionMessages {
///     pub attack: ConstMessage<0>,
///     pub run: ConstMessage<0>,
/// }
///
/// struct Messages {
///     pub actions: ActionMessages,
///     pub hello: ConstMessage<1>,
/// }
///
/// static LANG: RwLock<Lang> = RwLock::new(Lang::EN);
///
/// def_local_fmt!(
///     name = MESSAGES,
///     lang = Lang,
///     message = Messages {
///         actions: ActionMessages,
///     },
///     supplier = || *LANG.read().unwrap(),
///     file_type = "json",
///     lang_file = "doctest/lang.json"
/// );
///
/// assert_eq!(MESSAGES.hello.format(&["Rust"]), "Hello, world! Rust");
///
/// *LANG.write().unwrap() = Lang::JA;
///
/// assert_eq!(MESSAGES.hello.format(&["Rust"]), "こんにちは、世界！ Rust");
/// ```
#[proc_macro]
pub fn def_local_fmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as local_fmt_macros_internal::def_local_fmt::Args);

    local_fmt_macros_internal::def_local_fmt::generate(args)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn gen_const_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as local_fmt_macros_internal::util_macro::Args);

    args.to_token(true)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro]
pub fn gen_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as local_fmt_macros_internal::util_macro::Args);

    args.to_token(false)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
