use local_fmt::{gen_const_message, ConstMessage};

#[test]
fn test_unchecked() {
    let _: ConstMessage<1> = gen_const_message!(unchecked, "Hello! ", { 0 }, { 99 });
}

#[test]
#[should_panic = "index out of bounds: the len is 1 but the index is 99"]
fn test_unchecked_panic() {
    let message: ConstMessage<1> = gen_const_message!(unchecked, "Hello! ", { 0 }, { 99 });
    message.format(&["World!"]);
}

#[test]
fn test_1() {
    let message: ConstMessage<1> = gen_const_message!("Hello! ", { 0 });
    let text = message.format(&["World!"]);
    assert_eq!(text, "Hello! World!");
}

#[test]
fn test_2() {
    const HELLO: &str = "Hello";
    let message: ConstMessage<2> = gen_const_message!(HELLO, " ", { 0 }, " World! ", { 1 });

    let text = message.format(&["Beautiful", "Rust!"]);
    assert_eq!(text, "Hello Beautiful World! Rust!");
}
