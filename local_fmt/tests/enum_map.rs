#![cfg(feature = "macros")]

use local_fmt::EnumableMap;
use local_fmt_macros::Enumable;

#[derive(Enumable, Debug)]
enum Sample {
    A,
    B,
    C,
}

#[test]
fn new() {
    let map = EnumableMap::new(|k| match k {
        Sample::A => "Hey",
        Sample::B => "Kon",
        Sample::C => "Tya",
    });
    assert_eq!(map[Sample::A], "Hey");
    assert_eq!(map[Sample::B], "Kon");
    assert_eq!(map[Sample::C], "Tya");
}
