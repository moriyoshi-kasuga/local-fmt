#![cfg(feature = "macros")]

#[derive(Debug, PartialEq, Eq, Clone, Copy, local_fmt::macros::EnumIter)]
pub enum Key {
    HelloWorld,
    GameEndMessage,
}

#[test]
fn enum_iter() {
    let mut iter = <Key as local_fmt::EnumIter>::into_iter();
    assert_eq!(iter.next(), Some(Key::HelloWorld));
    assert_eq!(iter.next(), Some(Key::GameEndMessage));
    assert_eq!(iter.next(), None);
}
