use local_fmt::{gen_const_message, ConstMessage};

#[test]
fn test_1() {
    let _: ConstMessage<1> = gen_const_message!("Hello! ", { 0 });
}
