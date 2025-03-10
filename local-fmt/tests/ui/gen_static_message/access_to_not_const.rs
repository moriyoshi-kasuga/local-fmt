use local_fmt::{gen_static_message, StaticMessage};

fn main() {
    let world = "world";
    const _: StaticMessage<1> = gen_static_message!("Hello! {world}");
}
