#![cfg(feature = "macros")]
#![cfg(feature = "macros-toml")]

use std::sync::RwLock;

use enum_table::Enumable;
use local_fmt::{def_local_fmt, StaticMessage};

#[derive(Clone, Copy, Enumable)]
enum Lang {
    EN,
    JA,
}

struct WordsMessages {
    pub ownership: &'static str,
    pub compiler: &'static str,
}

struct Messages {
    pub words: WordsMessages,
    pub welcome: StaticMessage<1>,
    pub rust: StaticMessage<2>,
    pub goodbye: StaticMessage<1>,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::EN);

def_local_fmt!(
    name = MESSAGES,
    lang = Lang,
    message = Messages {
        words: WordsMessages,
    },
    supplier = || *LANG.read().unwrap(),
    file_type = "toml",
    lang_folder = "examples/lang/"
);

fn main() {
    let lang = std::env::args()
        .nth(1)
        .expect("Please specify lang, EN or JA");
    let lang = match lang.as_str() {
        "EN" => Lang::EN,
        "JA" => Lang::JA,
        _ => panic!("Invalid lang, expected EN or JA"),
    };
    let user = std::env::args().nth(2).expect("Please specify user");

    *LANG.write().unwrap() = lang;

    println!("{}", MESSAGES.welcome.format(&[&user]));

    println!(
        "{}",
        MESSAGES.rust.format(&[&user, MESSAGES.words.ownership])
    );
    println!(
        "{}",
        MESSAGES.rust.format(&[&user, MESSAGES.words.compiler])
    );

    println!("{}", MESSAGES.goodbye.format(&[&user]));
}
