use std::{collections::HashMap, sync::RwLock};

use local_fmt::{ConstMessage, LocalFmt, LoadFileUtil};

#[derive(serde::Deserialize)]
pub struct Messages {
    pub hello: ConstMessage<1>,
}

impl LoadFileUtil for Messages {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Lang {
    JA,
    EN,
    Other,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::JA);

#[allow(clippy::unwrap_used, clippy::print_stdout)]
fn main() {
    let mut messages: HashMap<Lang, Messages> = HashMap::new();

    messages.insert(
        Lang::JA,
        Messages::load_from_file(toml::from_str, "./local-fmt/examples/ja.toml").unwrap(),
    );
    messages.insert(
        Lang::EN,
        Messages::load_from_file(toml::from_str, "./local-fmt/examples/en.toml").unwrap(),
    );

    let local = LocalFmt::new(messages, || *LANG.read().unwrap());

    {
        let message = local.get_message().hello.format(&["mori"]);

        assert_eq!(message, "こんにちは mori さん");
        println!("{}", message);
    }

    *LANG.write().unwrap() = Lang::EN;

    {
        let message = local.get_message().hello.format(&["mori"]);

        assert_eq!(message, "Hello mori!");
        println!("{}", message);
    }
}
