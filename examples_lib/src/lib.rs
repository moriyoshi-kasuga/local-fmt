#[cfg(test)]
mod tests {

    use local_fmt::macros::{def_local_fmt, ConvertStr};

    #[derive(Default, ConvertStr, Debug, Hash, Eq, PartialEq, Clone, Copy)]
    pub enum Lang {
        JA,
        #[default]
        EN,
    }

    #[derive(ConvertStr, Hash, Eq, PartialEq, Clone, Copy)]
    pub enum Key {
        Hello,
        Goodbye,
    }

    def_local_fmt!(ident = TRANSLATOR, lang = Lang, key = Key);

    #[test]
    fn test() {
        // initialize
        let _ = &*TRANSLATOR;

        assert_eq!(TRANSLATOR.format(Lang::EN, Key::Hello, &[]), "Hello world");
        assert_eq!(
            TRANSLATOR.format(Lang::JA, Key::Hello, &[]),
            "こんにちは世界"
        );
        assert_eq!(
            TRANSLATOR.format(Lang::EN, Key::Goodbye, &[("human", "mori")]),
            "Goodbye mori"
        );
    }
}
