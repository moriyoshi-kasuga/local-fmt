use local_fmt::macros::{def_local_fmt, ConvertStr, Enumable};

#[derive(Default, ConvertStr, Debug, Hash, Eq, PartialEq, Clone, Copy, Enumable)]
pub enum Lang {
    JA,
    #[default]
    EN,
}

#[derive(ConvertStr, Debug, Hash, Eq, PartialEq, Clone, Copy, Enumable)]
pub enum Key {
    Hello,
    Goodbye,
    GameStart,
}

def_local_fmt!(
    ident = TRANSLATOR,
    lang = Lang,
    key = Key,
    app_file = "app.toml"
);

#[test]
fn test_translator() {
    // initialize
    let _ = &*TRANSLATOR;
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // skip program name
    let lang: Lang = match args.next() {
        Some(lang) => match lang.parse() {
            Ok(lang) => lang,
            Err(_) => panic!("Error: invalid lang"),
        },
        None => panic!("Error: no lang"),
    };
    let name = match args.next() {
        Some(name) => name,
        None => panic!("Error: no name"),
    };

    println!("{}", TRANSLATOR.format(lang, Key::Hello, &[]));
    println!("{}", TRANSLATOR.format(lang, Key::GameStart, &[]));
    println!(
        "{}",
        TRANSLATOR.format(lang, Key::Goodbye, &[("human", &name)])
    );
}
