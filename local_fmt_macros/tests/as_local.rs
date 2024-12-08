use local_fmt_macros::AsLocal;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, AsLocal)]
pub enum Lang {
    JA,
    EN,
    ZH,
}
#[test]
fn as_local() {
    assert_eq!(Into::<&'static str>::into(Lang::JA), "ja");
    assert_eq!(Into::<&'static str>::into(Lang::EN), "en");
    assert_eq!(TryInto::<Lang>::try_into("en").unwrap(), Lang::EN);
    assert_eq!(TryInto::<Lang>::try_into("ja").unwrap(), Lang::JA);
}
