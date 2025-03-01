#![cfg(feature = "derive")]

use std::sync::RwLock;

use enum_table::{EnumTable, Enumable};
use local_fmt::{
    def_local_fmt, gen_const_message, ConstMessage, LangSupplier, LoadFileUtil, LocalFmt,
};

#[derive(serde::Deserialize)]
pub struct Messages {
    pub hello: ConstMessage<1>,
    pub goodbye: ConstMessage<1>,
}

impl LoadFileUtil for Messages {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enumable)]
#[repr(u8)]
pub enum Lang {
    JA,
    EN,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::JA);

// def_local_fmt!(
//     name = WORDS,
//     lang = Lang,
//     message = Messages,
//     supplier = LangSupplier::Dynamic(|| *LANG.read().unwrap()),
//     lang_folder = ".",
// );

#[allow(clippy::unwrap_used, clippy::print_stdout)]
fn main() {
    let messages: EnumTable<Lang, Messages, { Lang::COUNT }> =
        EnumTable::new_with_fn(|lang| match lang {
            Lang::JA => Messages {
                hello: gen_const_message!(1, "こんにちは", { 0 }, "さん",),
                goodbye: gen_const_message!(1, "さようなら", { 0 }),
            },
            Lang::EN => {
                Messages::load_from_file(toml::from_str, "./local-fmt/examples/en.toml").unwrap()
            }
        });

    let local = LocalFmt::new(messages, LangSupplier::Dynamic(|| *LANG.read().unwrap()));

    {
        let message = local.hello.format(&["mori"]);

        assert_eq!(message, "こんにちは mori さん");
        println!("{}", message);
    }

    *LANG.write().unwrap() = Lang::EN;

    {
        let message = local.hello.format(&["mori"]);

        assert_eq!(message, "Hello mori!");
        println!("{}", message);
    }
}
