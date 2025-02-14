use local_fmt::{gen_const_message, ConstMessage};

#[test]
fn test_unchecked() {
    let _: ConstMessage<1> = unsafe { gen_const_message!(unchecked "Hello! ", { 0 }, { 99 }) };
}

#[test]
fn test_1_success() {
    let _: ConstMessage<1> = gen_const_message!("Hello! ", { 0 });
}

#[test]
#[should_panic]
fn test_2_failed() {
    let _: ConstMessage<2> = gen_const_message!("Hello! ", { 0 }, " World!");
}
