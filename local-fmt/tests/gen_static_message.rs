use local_fmt::{gen_static_message, StaticMessage};

#[test]
fn arg_1() {
    const MESSAGE: StaticMessage<1> = gen_static_message!("Hello! {0}");
    let text = MESSAGE.format(&["World!"]);
    assert_eq!(text, "Hello! World!");
}

#[test]
fn arg_2() {
    const HELLO: &str = "Hello";
    const MESSAGE: StaticMessage<2> = gen_static_message!("{HELLO} {0} World! {1}");

    let text = MESSAGE.format(&["Beautiful", "Rust!"]);
    assert_eq!(text, "Hello Beautiful World! Rust!");
}

#[test]
fn duplicate_arg() {
    const HELLO: &str = "Hello";
    const MESSAGE: StaticMessage<1> = gen_static_message!("{HELLO} {0} World! {0}");

    let text = MESSAGE.format(&["Beautiful"]);
    assert_eq!(text, "Hello Beautiful World! Beautiful");
}

#[test]
fn with_u_number() {
    const NUM: usize = 123456789;
    const MESSAGE: StaticMessage<1> = gen_static_message!("Hello! {0} {u:NUM}");
    let text = MESSAGE.format(&["World!"]);
    assert_eq!(text, "Hello! World! 123456789");
}

#[test]
fn with_i_number() {
    const NUM: i32 = -123456789;
    const MESSAGE: StaticMessage<1> = gen_static_message!("Hello! {0} {i:NUM}");
    let text = MESSAGE.format(&["World!"]);
    assert_eq!(text, "Hello! World! -123456789");
}
