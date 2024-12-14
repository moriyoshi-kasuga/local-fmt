#![cfg(all(feature = "macros", not(any(feature = "selected", feature = "global"))))]

use local_fmt::EnumableMap;
use local_fmt_macros::Enumable;

#[derive(Enumable, Debug, Clone, Copy)]
pub enum Key {
    HelloWorld,
    GameEndMessage,
}

#[derive(Enumable, Debug, Clone, Copy)]
pub enum Lang {
    English,
    Japanese,
}

#[test]
fn local_fmt() {
    let en_key_map = EnumableMap::new(|k| match k {
        Key::HelloWorld => "Hello, World! %{name}",
        Key::GameEndMessage => "Game Over! %{name}",
    });
    let ja_key_map = EnumableMap::new(|k| match k {
        Key::HelloWorld => "こんにちは、世界！ %{name}",
        Key::GameEndMessage => "ゲームオーバー！ %{name}",
    });
    let lang_map = EnumableMap::new(|k| match k {
        Lang::English => en_key_map.clone(),
        Lang::Japanese => ja_key_map.clone(),
    });

    let fmt = local_fmt::LocalFmt::new(lang_map);

    assert_eq!(
        fmt.format(Lang::English, Key::HelloWorld, &[("name", "Mori")]),
        "Hello, World! Mori"
    );
    assert_eq!(
        fmt.format(Lang::Japanese, Key::GameEndMessage, &[("name", "Mori")]),
        "ゲームオーバー！ Mori"
    );
}
