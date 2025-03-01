use std::str::FromStr;

use local_fmt::ConstMessage;

#[test]
fn normal() {
    let text = "Hello {0}";
    let message = ConstMessage::<1>::from_str(text).unwrap();
    let text = message.format(&["World!"]);
    assert_eq!(text, "Hello World!");
}

#[test]
fn failed() {
    let text = "Hello {1}";
    let message = ConstMessage::<1>::from_str(text).unwrap_err();
    assert_eq!(
        message,
        local_fmt::ConstMessageError::InvalidNumber { number: 1, n: 1 }
    );
}

#[test]
fn with_backslash() {
    let text = "Hey \\{1} {1} {0}";
    let message = ConstMessage::<2>::from_str(text).unwrap();
    let text = message.format(&["World!", "Rust!"]);
    assert_eq!(text, "Hey {1} Rust! World!");
}
