#![cfg(feature = "serde")]

use local_fmt::{AllocMessage, AllocMessageFormat, RefMessageFormat, StaticMessage};

#[derive(serde::Serialize)]
struct Static {
    message: StaticMessage<1>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Alloc {
    message: AllocMessage<1>,
}

#[test]
fn ser_static() {
    let static_struct = Static {
        message: StaticMessage::<1>::new_panic(&[
            RefMessageFormat::RefText("Hello, world! "),
            RefMessageFormat::Placeholder(0),
        ]),
    };

    let text = toml::to_string(&static_struct).unwrap();
    assert_eq!(text, "message = \"Hello, world! {0}\"\n");
}

#[test]
fn ser_alloc() {
    let alloc_struct = Alloc {
        message: AllocMessage::new_panic(vec![
            AllocMessageFormat::AllocText("Hello, alloc! ".to_string()),
            AllocMessageFormat::Placeholder(0),
        ]),
    };

    let text = toml::to_string(&alloc_struct).unwrap();
    assert_eq!(text, "message = \"Hello, alloc! {0}\"\n");
}

#[test]
fn de_alloc() {
    let text = "message = 'Hello, alloc! {0}'";
    let test: Alloc = toml::from_str(text).unwrap();

    let expected_message = AllocMessage::new_panic(vec![
        AllocMessageFormat::AllocText("Hello, alloc! ".to_string()),
        AllocMessageFormat::Placeholder(0),
    ]);

    assert_eq!(test.message, expected_message);
}
