use local_fmt_macros::gen_static_message;
use sealed::Sealed;

use crate::{panic_builder, StaticMessage};

mod sealed {
    use crate::StaticMessage;

    pub trait Sealed {}

    impl Sealed for &'static str {}
    impl<const N: usize> Sealed for StaticMessage<N> {}
}

/// Trait for validating static message arguments at compile time.
/// 
/// This trait is used internally by the `def_local_fmt` macro to ensure that
/// message arguments have compatible types. It provides compile-time validation
/// that prevents type mismatches when defining localized messages.
/// 
/// # Implementation Details
/// 
/// The trait uses an associated constant `IS_INVALID` to signal validation errors.
/// When `IS_INVALID` is `Some`, it contains an error message that will be displayed
/// at compile time. When it's `None`, the type conversion is valid.
/// 
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait CheckStaticMessageArg<To>: Sealed {
    /// Contains an error message if the type conversion is invalid, or None if valid.
    const IS_INVALID: Option<StaticMessage<2>>;
}

/// Checks if the message argument is valid.
/// If the argument is invalid, a panic is raised with a detailed error message.
/// Otherwise, the argument is returned.
/// 
/// # Safety
/// This function uses unsafe transmute operations, but only after compile-time validation
/// that ensures the From and To types are compatible. The CheckStaticMessageArg trait
/// provides compile-time guarantees about type compatibility.
#[track_caller]
pub const fn check_static_message_arg<To, From>(
    lang: &'static str,
    key: &'static str,
    from: From,
) -> To
where
    From: CheckStaticMessageArg<To>,
{
    if let Some(message) = From::IS_INVALID {
        panic_builder!(message, [lang], [key]);
    }
    
    // SAFETY: The CheckStaticMessageArg trait ensures that From and To types
    // are compatible for this conversion. The trait implementation validates
    // that the size and layout requirements are met at compile time.
    // 
    // For the specific cases implemented:
    // - &'static str -> &'static str: identity conversion, always safe
    // - StaticMessage<N> -> StaticMessage<M> where N == M: same type, safe
    // 
    // The ManuallyDrop wrapper prevents double-drops while preserving the
    // memory layout for the transmute operation.
    unsafe { 
        use std::mem::{ManuallyDrop, transmute_copy};
        transmute_copy::<ManuallyDrop<From>, To>(&ManuallyDrop::new(from))
    }
}

impl CheckStaticMessageArg<&'static str> for &'static str {
    const IS_INVALID: Option<StaticMessage<2>> = None;
}

impl<const N: usize, const M: usize> CheckStaticMessageArg<StaticMessage<N>> for StaticMessage<M> {
    const IS_INVALID: Option<StaticMessage<2>> = if N == M {
        None
    } else {
        Some(gen_static_message!(
            "Error: A message with {u:M} arguments was expected in the language '{0}', ",
            "but received a message with {u:N} arguments for the key '{1}'. ",
            "Please check the message definition and ensure the correct number of arguments."
        ))
    };
}

impl<const N: usize> CheckStaticMessageArg<&'static str> for StaticMessage<N> {
    const IS_INVALID: Option<StaticMessage<2>> = Some(gen_static_message!(
        "Error: A message with {u:N} arguments was expected in the language '{0}', ",
        "but received a message with no arguments for the key '{1}'. ",
        "Please check the message definition and ensure the correct number of arguments."
    ));
}

impl<const N: usize> CheckStaticMessageArg<StaticMessage<N>> for &'static str {
    const IS_INVALID: Option<StaticMessage<2>> = Some(gen_static_message!(
        "Error: A message with {u:N} arguments was expected in the language '{0}', ",
        "but received a message with no arguments for the key '{1}'. ",
        "Please check the message definition and ensure the correct number of arguments."
    ));
}
