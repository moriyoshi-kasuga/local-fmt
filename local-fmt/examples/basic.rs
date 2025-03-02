use std::sync::RwLock;

use enum_table::Enumable;
use local_fmt::def_local_fmt;
use local_fmt::ConstMessage;

#[derive(Clone, Copy, Enumable)]
enum Lang {
    EN,
    JA,
}

struct Messages {
    pub welcome: ConstMessage<1>,
    pub rust: ConstMessage<2>,
    pub goodbye: ConstMessage<1>,
}

static LANG: RwLock<Lang> = RwLock::new(Lang::EN);

#[allow(clippy::unwrap_used)]
fn get_lang() -> Lang {
    *LANG.read().unwrap()
}

def_local_fmt!(
    name = MESSAGES,
    lang = Lang,
    message = Messages,
    dynamic_supplier = get_lang,
    lang_folder = "examples/lang/"
);

#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]
fn main() {
    let lang = std::env::args().nth(1).expect("Please specify lang");
    let lang = match lang.as_str() {
        "EN" => Lang::EN,
        "JA" => Lang::JA,
        _ => panic!("Invalid lang"),
    };
    let user = std::env::args().nth(2).expect("Please specify user");

    *LANG.write().unwrap() = lang;

    println!("{}", MESSAGES.welcome.format(&[&user]));

    println!("{}", MESSAGES.rust.format(&[&user, "ownership"]));
    println!("{}", MESSAGES.rust.format(&[&user, "compiler"]));

    println!("{}", MESSAGES.goodbye.format(&[&user]));
}
