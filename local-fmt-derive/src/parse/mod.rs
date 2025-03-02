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

impl Parse for MessageToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut values = Vec::new();

        let mut cursor = input.cursor();
        while !cursor.eof() {
            #[allow(clippy::unwrap_used)]
            let (tree, next) = cursor.token_tree().unwrap();
            cursor = next;

            // match tree {
            // }
        }

        Ok(Self { values })
    }
}
