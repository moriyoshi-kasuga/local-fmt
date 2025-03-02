// compiletime check macro
/// The `gen_const_message` macro is used to generate a constant message format at compile time.
/// It supports both static text and argument placeholders, allowing for efficient message formatting.
///
/// # Syntax
/// - `gen_const_message!(text, {arg}, ident, expr)`
///   - `text`: A string literal representing static text.
///   - `{arg}`: A placeholder for an argument, specified as a literal number.
///   - `ident`: An identifier that will be treated as static text.
///   - `expr`: An expression that will be treated as static text.
///
/// # Examples
/// ```rust
/// use local_fmt::gen_const_message;
///
/// const HELLO: &str = "Hello";
///
/// // Create a constant message with static text and an argument placeholder.
/// const MESSAGE: local_fmt::ConstMessage<1> = gen_const_message!(HELLO, " ", {0}, "!");
///
/// // This will format the message with the argument "World".
/// assert_eq!(MESSAGE.format(&["World"]), "Hello World!");
#[macro_export]
macro_rules! gen_const_message {
     (@gen $text:literal) => {
         $crate::MessageFormat::StaticText($text)
     };
     (@gen {$number:literal}) => {
         $crate::MessageFormat::Arg($number)
     };
     (@gen $ident:ident) => {
         $crate::MessageFormat::StaticText($ident)
     };
     (@gen $expr:expr) => {
         $crate::MessageFormat::StaticText($expr)
     };
     (unchecked, $($tt:tt),*) => {
         $crate::ConstMessage::Static(&[$(gen_const_message!(@gen $tt)),*])
     };
     ($($tt:tt),* $(,)?) => {
        const {$crate::ConstMessage::new_static(
            &[$($crate::gen_const_message!(@gen $tt)),*]
        )}
     }
 }

// useable string macro
/// The `gen_message` macro is used to generate a message format that can be used at runtime.
/// It supports both static text and argument placeholders, allowing for dynamic message formatting.
///
/// # Syntax
/// - `gen_message!(text, {arg}, ident, expr)`
///   - `text`: A string literal representing static text.
///   - `{arg}`: A placeholder for an argument, specified as a literal number.
///   - `ident`: An identifier that will be treated as dynamic text.
///   - `expr`: An expression that will be treated as dynamic text.
///
/// # Examples
/// ```rust
/// use local_fmt::gen_message;
///
/// let hello = "Hello".to_string();
///
/// // Create a message with static text and an argument placeholder.
/// let message = gen_message!(hello, " ", {0}, "!");
/// // returned Result<ConstMessage, ConstMessageError>
/// let message = message.unwrap();
///
/// // This will format the message with the argument "World".
/// assert_eq!(message.format(&["World"]), "Hello World!");
/// ```
#[macro_export]
macro_rules! gen_message {
     (@gen $text:literal) => {
         $crate::MessageFormat::StaticText($text)
     };
     (@gen {$number:literal}) => {
         $crate::MessageFormat::Arg($number)
     };
     (@gen $ident:ident) => {
         $crate::MessageFormat::Text($ident)
     };
     (@gen $expr:expr) => {
         $crate::MessageFormat::Text($expr)
     };
     (unchecked, $($tt:tt),* $(,)?) => {
         $crate::ConstMessage::Vec(vec![$(gen_message!(@gen $tt)),*])
     };
     ($($tt:tt),* $(,)?) => {
         $crate::ConstMessage::new(vec![$(gen_message!(@gen $tt)),*])
     }
 }
