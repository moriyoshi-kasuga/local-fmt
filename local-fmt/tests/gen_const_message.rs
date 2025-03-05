use local_fmt::{gen_const_message, ConstMessage};

#[test]
fn test_unchecked() {
    // let _: ConstMessage<1> = gen_const_message!(unchecked, "Hello! ", { 0 }, { 99 });
    const _: ConstMessage<10> = gen_const_message!(unchecked, "Hello! {0} {9}");
}

#[test]
fn test_unchecked_no_error() {
    const MESSAGE: ConstMessage<3> = gen_const_message!(unchecked, "Hello! {0} {2}");
    let text = MESSAGE.format(&["World!", "Rust!", "Beautiful"]);
    assert_eq!(text, "Hello! World! Beautiful");
}

#[test]
fn test_1() {
    const MESSAGE: ConstMessage<1> = gen_const_message!("Hello! {0}");
    let text = MESSAGE.format(&["World!"]);
    assert_eq!(text, "Hello! World!");
}

#[test]
fn test_2() {
    const HELLO: &str = "Hello";
    const MESSAGE: ConstMessage<2> = gen_const_message!("{HELLO} {0} World! {1}");

    let text = MESSAGE.format(&["Beautiful", "Rust!"]);
    assert_eq!(text, "Hello Beautiful World! Rust!");
}

#[test]
fn duplicate_arg() {
    const HELLO: &str = "Hello";
    const MESSAGE: ConstMessage<1> = gen_const_message!("{HELLO} {0} World! {0}");

    let text = MESSAGE.format(&["Beautiful"]);
    assert_eq!(text, "Hello Beautiful World! Beautiful");
}
