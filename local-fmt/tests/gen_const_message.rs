use local_fmt::{gen_const_message, ConstMessage};

#[test]
fn test_unchecked() {
    let _: ConstMessage<1> = unsafe { gen_const_message!(unchecked, "Hello! ", { 0 }, { 99 }) };
}

#[test]
fn test_1() {
    let _ = gen_const_message!(1, "Hello! ", { 0 });
}

#[test]
fn test_2() {
    let _ = gen_const_message!(2, "Hello! ", { 0 }, " World!", { 1 });
}
