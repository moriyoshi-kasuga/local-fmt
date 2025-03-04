use std::str::FromStr;

use syn::{parse::Parse, Ident};

pub(crate) enum MessageTokenValue {
    StaticText(String),
    PlaceholderArg(usize),

    // in const, it's expected to be a &'static str
    // in not const, it's expected to be a String
    PlaceholderIdent(Ident),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum MessageTokenValueError {
    #[error("Placeholder number {0} is not found in the message. The hiest number found is {1}")]
    NotFound(usize, usize),
    #[error("Placeholder number {0} is out of range. The hiest number found is {1}")]
    OutOfRange(usize, usize),
}

pub(crate) struct MessageToken {
    values: Vec<MessageTokenValue>,
}

impl FromStr for MessageToken {
    type Err = syn::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}
