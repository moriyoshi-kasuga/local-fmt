#![cfg(feature = "serde")]

use local_fmt::{gen_const_message, ConstMessage, MessageFormat};

#[derive(serde::Serialize, serde::Deserialize)]
struct Test {
    hello: ConstMessage<1>,
}

#[test]
fn ser() {
    let message = gen_const_message!("Hello! ", { 0 });
    let test = Test { hello: message };
    let text = toml::to_string(&test).unwrap();
    assert_eq!(text, "hello = \"Hello! {0}\"\n");
}

#[test]
fn de() {
    let text = "hello = 'Hello! {0}'";
    let test: Test = toml::from_str(text).unwrap();

    let text = ConstMessage::<1>::Vec(vec![
        MessageFormat::Text("Hello! ".to_string()),
        MessageFormat::Arg(0),
    ]);

    assert_eq!(test.hello, text);
}
