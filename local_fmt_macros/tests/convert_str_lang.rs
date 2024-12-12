use local_fmt_macros::ConvertStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, ConvertStr)]
pub enum Lang {
    JA,
    EN,
    ZH,
}

#[test]
fn convert_str_into() {
    assert_eq!(Into::<&'static str>::into(Lang::JA), "ja");
    assert_eq!(Into::<&'static str>::into(Lang::EN), "en");
    assert_eq!(Into::<&'static str>::into(Lang::ZH), "zh");
}

#[test]
fn convert_str_try_from() {
    assert_eq!(TryInto::<Lang>::try_into("en").unwrap(), Lang::EN);
    assert_eq!(TryInto::<Lang>::try_into("ja").unwrap(), Lang::JA);
    assert_eq!(TryInto::<Lang>::try_into("zh").unwrap(), Lang::ZH);
}

#[test]
fn convert_str_error() {
    assert_eq!(
        TryInto::<Lang>::try_into("eN").unwrap_err(),
        "cannot convert eN to Lang"
    );
    assert_eq!(
        TryInto::<Lang>::try_into("jp").unwrap_err(),
        "cannot convert jp to Lang"
    );
}
