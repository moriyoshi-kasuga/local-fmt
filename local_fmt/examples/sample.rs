use std::{collections::HashMap, sync::RwLock};

use local_fmt::{gen_const_message, ConstMessage, LocalFmt};

pub struct Messages {
    pub hello: ConstMessage<1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Lang {
    JA,
    EN,
}

#[allow(clippy::unwrap_used)]
fn main() {
    let mut messages = HashMap::new();
    messages.insert(
        Lang::JA,
        Messages {
            hello: gen_const_message!(1, "こんにちは ", { 0 }, " さん"),
        },
    );

    messages.insert(
        Lang::EN,
        Messages {
            hello: gen_const_message!(1, "hello ", { 0 }),
        },
    );

    static LANG: RwLock<Lang> = RwLock::new(Lang::JA);

    let local = LocalFmt::new(messages, || *LANG.read().unwrap());

    {
        let message = local.get_message().hello.format(&["mori"]);

        assert_eq!(message, "こんにちは mori さん");
    }

    *LANG.write().unwrap() = Lang::EN;

    {
        let message = local.get_message().hello.format(&["mori"]);

        assert_eq!(message, "hello mori");
    }
}
