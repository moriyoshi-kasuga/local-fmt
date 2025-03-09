use crate::panic_builder;

pub mod alloc;
pub use alloc::*;

pub mod refer;
pub use refer::*;

/// Represents errors that can occur when working with constant messages.
///
/// This enum provides detailed error information for invalid or missing argument numbers
/// in constant messages.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum CreateMessageError {
    /// Error indicating that an argument number is out of the allowed range.
    ///
    /// This error occurs when an argument number is provided that is not within the valid range
    /// of 0 to N-1, where N is the number of expected arguments.
    #[error("Invalid argument number: {number} is out of the allowed range (0 <= number < {n}).")]
    InvalidNumber { number: usize, n: usize },

    /// Error indicating that a required argument number is missing.
    ///
    /// This error occurs when an expected argument number is not found within the valid range
    /// of 0 to N-1, where N is the number of expected arguments.
    #[error("Missing argument number: {number} is not found within the allowed range (0 <= number < {n}).")]
    WithoutNumber { number: usize, n: usize },
    /// Error indicating that a placeholder is empty.
    ///
    /// This error occurs when a placeholder is empty.
    /// This can happen when a placeholder is found without a number.
    #[error("Empty placeholder found: a placeholder was opened but not closed properly. Ensure all placeholders are correctly formatted.")]
    EmptyPlaceholder,
}

impl CreateMessageError {
    #[track_caller]
    pub const fn panic(&self) -> ! {
        match self {
            Self::InvalidNumber { number, n } => {
                panic_builder!(
                    ["Invalid argument number: "],
                    [u = *number],
                    [" is out of the allowed range (0 <= number < "],
                    [u = *n],
                    [")."]
                )
            }
            Self::WithoutNumber { number, n } => {
                panic_builder!(
                    ["Missing argument number: "],
                    [u = *number],
                    [" is not found within the allowed range (0 <= number < "],
                    [u = *n],
                    [")."]
                )
            }
            Self::EmptyPlaceholder => {
                panic!("Empty placeholder found: a placeholder was opened but not closed properly. Ensure all placeholders are correctly formatted.")
            }
        }
    }
}
