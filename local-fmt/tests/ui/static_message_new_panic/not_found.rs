use local_fmt::{RefMessageFormat, StaticMessage};

const _: StaticMessage<1> =
    StaticMessage::<1>::new_panic(&[RefMessageFormat::RefText("Hello, world! ")]);

fn main() {}
