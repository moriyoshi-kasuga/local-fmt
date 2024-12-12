use local_fmt_macros::ConvertStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, ConvertStr)]
pub enum Key {
    HelloWorld,
    GameEndMessage,
}

#[test]
fn convert_str_into() {
    assert_eq!(Into::<&'static str>::into(Key::HelloWorld), "hello_world");
    assert_eq!(
        Into::<&'static str>::into(Key::GameEndMessage),
        "game_end_message"
    );
}

#[test]
fn convert_str_try_from() {
    assert_eq!(
        TryInto::<Key>::try_into("hello_world").unwrap(),
        Key::HelloWorld
    );
    assert_eq!(
        TryInto::<Key>::try_into("game_end_message").unwrap(),
        Key::GameEndMessage
    );
}
