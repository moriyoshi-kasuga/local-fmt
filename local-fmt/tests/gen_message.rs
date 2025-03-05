use local_fmt::{gen_message, ConstMessage};

#[test]
fn test_unchecked() {
    let _: ConstMessage<3> = gen_message!(unchecked, "Hello! {0} {2}");
}

#[test]
fn test_unchecked_no_error() {
    let message = gen_message!(unchecked, "Hello! {0} {2}");
    let text = message.format(&["World!", "Rust!", "Beautiful"]);
    assert_eq!(text, "Hello! World! Beautiful");
}

#[test]
fn test_1() {
    let result = gen_message!("Hello! {0}");
    let text = result.format(&["World!"]);
    assert_eq!(text, "Hello! World!");
}

#[test]
fn test_2() {
    let hey = String::from("hey");
    let result: ConstMessage<2> = gen_message!("{hey} {0} World! {1}");

    let text = result.format(&["Beautiful", "Rust!"]);
    assert_eq!(text, "hey Beautiful World! Rust!");
}
