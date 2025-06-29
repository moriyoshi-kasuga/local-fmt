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
/// * `supplier` - The language supplier, a function of type `fn() -> Lang`.
/// It determines how the current language is selected dynamically at runtime.
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
/// ## Message Nesting
/// * The `message` struct can be nested, allowing for organized grouping of related messages.
///   For example, you can have a struct for action messages nested within a main message struct.
///   This helps in maintaining a clean and structured message hierarchy.
///
/// ## Static String Loading
/// * If a message does not require any arguments, it can be loaded as a `&'static str`.
///   This allows for efficient handling of static messages without the need for formatting.
///   Simply define the message field as `&'static str` in your message struct.
///
/// # Example
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
/// use local_fmt::{def_local_fmt, StaticMessage};
///
/// #[derive(Clone, Copy, Enumable)]
/// enum Lang {
///     EN,
///     JA,
/// }
///
/// struct Messages {
///     pub hello: StaticMessage<1>,
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
/// use local_fmt::{def_local_fmt, StaticMessage, LocalFmt};
/// use std::sync::RwLock;
///
/// #[derive(Clone, Copy, Enumable)]
/// enum Lang {
///    EN,
///    JA,
/// }
///
/// struct ActionMessages {
///     pub attack: &'static str,
///     pub run: &'static str,
/// }
///
/// struct Messages {
///     pub actions: ActionMessages,
///     pub hello: StaticMessage<1>,
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

/// Generates a static message with placeholders for arguments.
///
/// This macro creates a `StaticMessage` that can be used to format strings with
/// a fixed number of arguments. The placeholders in the message are denoted by
/// `{0}`, `{1}`, etc., which correspond to the arguments provided during formatting.
///
/// # Notes
///
/// - The number of placeholders in the message must match the number of arguments
///   specified in the `StaticMessage` type.
/// - The macro supports using constants within the message string.
/// - You can include numeric constants directly in the message using the `{u:}` or `{i:}` syntax
///   for unsigned and signed integers, respectively.
///
/// # Examples
///
/// ```rust
/// use local_fmt::{gen_static_message, StaticMessage};
///
/// // Example with argument
/// {
///     const MESSAGE: StaticMessage<1> = gen_static_message!("Hello! {0}");
///     let text = MESSAGE.format(&["World!"]);
///     assert_eq!(text, "Hello! World!");
/// }
///
/// // Example with const placeholder
/// {
///     const HELLO: &str = "Hello";
///     const MESSAGE: StaticMessage<2> = gen_static_message!("{HELLO} {0} World! {1}");
///     let text = MESSAGE.format(&["Beautiful", "Rust!"]);
///     assert_eq!(text, "Hello Beautiful World! Rust!");
/// }
///
/// // Example with duplicate arguments
/// {
///     const MESSAGE: StaticMessage<1> = gen_static_message!("{0} World! {0}");
///     let text = MESSAGE.format(&["Beautiful"]);
///     assert_eq!(text, "Beautiful World! Beautiful");
/// }
///
/// // Example with unsigned number
/// {
///     const NUM: usize = 123456789;
///     const MESSAGE: StaticMessage<1> = gen_static_message!("Hello! {0} {u:NUM}");
///     let text = MESSAGE.format(&["World!"]);
///     assert_eq!(text, "Hello! World! 123456789");
/// }
///
/// // Example with signed number
/// {
///     const NUM: i32 = -123456789;
///     const MESSAGE: StaticMessage<1> = gen_static_message!("Hello! {0} {i:NUM}");
///     let text = MESSAGE.format(&["World!"]);
///     assert_eq!(text, "Hello! World! -123456789");
/// }
#[proc_macro]
pub fn gen_static_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as local_fmt_macros_internal::util_macro::Args);

    args.to_token(true)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Generates an allocatable message with placeholders for arguments.
///
/// This macro creates an `AllocMessage` that can be used to format strings with
/// a fixed number of arguments. The placeholders in the message are denoted by
/// `{0}`, `{1}`, etc., which correspond to the arguments provided during formatting.
///
/// # Notes
///
/// - The number of placeholders in the message must match the number of arguments
///   specified in the `AllocMessage` type.
/// - The macro supports using ident within the message string.
///
/// # Examples
///
/// ```rust
/// use local_fmt::{gen_alloc_message, AllocMessage};
///
/// // Example with argument
/// {
///     let message: AllocMessage<1> = gen_alloc_message!("Hello! {0}");
///     let text = message.format(&["World!"]);
///     assert_eq!(text, "Hello! World!");
/// }
///
/// // Example with string placeholder
/// {
///     let hello: String = "Hello".to_string();
///     let message: AllocMessage<2> = gen_alloc_message!("{hello} {0} World! {1}");
///     let text = message.format(&["Beautiful", "Rust!"]);
///     assert_eq!(text, "Hello Beautiful World! Rust!");
/// }
///
/// // Example with duplicate arguments
/// {
///     let message: AllocMessage<1> = gen_alloc_message!("{0} World! {0}");
///     let text = message.format(&["Beautiful"]);
///     assert_eq!(text, "Beautiful World! Beautiful");
/// }
#[proc_macro]
pub fn gen_alloc_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as local_fmt_macros_internal::util_macro::Args);

    args.to_token(false)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
