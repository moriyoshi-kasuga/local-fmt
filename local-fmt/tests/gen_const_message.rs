use local_fmt::gen_const_message;

#[test]
fn test_unchecked() {
    let _ = unsafe { gen_const_message!(unchecked, 1, "Hello! ", { 0 }, { 99 }) };
}

#[test]
#[should_panic = "index out of bounds: the len is 1 but the index is 99"]
fn test_unchecked_panic() {
    let message = unsafe { gen_const_message!(unchecked, 1, "Hello! ", { 0 }, { 99 }) };
    message.format(&["World!"]);
}

#[test]
fn test_1() {
    let _ = gen_const_message!(1, "Hello! ", { 0 });
}

#[test]
fn test_2() {
    let _ = gen_const_message!(2, "Hello! ", { 0 }, " World!", { 1 });
}
