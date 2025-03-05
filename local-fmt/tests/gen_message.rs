use local_fmt::{gen_message, ConstMessage};

#[test]
fn arg_1() {
    let result = gen_message!("Hello! {0}");
    let text = result.format(&["World!"]);
    assert_eq!(text, "Hello! World!");
}

#[test]
fn arg_2() {
    let hey = String::from("hey");
    let result: ConstMessage<2> = gen_message!("{hey} {0} World! {1}");

    let text = result.format(&["Beautiful", "Rust!"]);
    assert_eq!(text, "hey Beautiful World! Rust!");
}
