use local_fmt::{gen_message, ConstMessage, ConstMessageError};

#[test]
fn test_unchecked() {
    let _: ConstMessage<1> = gen_message!(unchecked, "Hello! ", { 0 }, { 99 });
}

#[test]
#[should_panic = "index out of bounds: the len is 1 but the index is 99"]
fn test_unchecked_panic() {
    let message = gen_message!(unchecked, "Hello! ", { 0 }, { 99 });
    message.format(&["World!"]);
}

#[test]
fn test_1() {
    let result = gen_message!("Hello! ", { 0 }).unwrap();
    let text = result.format(&["World!"]);
    assert_eq!(text, "Hello! World!");
}

#[test]
fn test_2() {
    let hey = String::from("hey");
    let result: Result<ConstMessage<2>, ConstMessageError> =
        gen_message!(hey, { 0 }, " World!", { 99 });

    assert_eq!(
        result.unwrap_err(),
        ConstMessageError::InvalidNumber { number: 99, n: 2 }
    );
}
