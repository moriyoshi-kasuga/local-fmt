use local_fmt::LoadFileUtil;

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Test {
    pub title: String,
    pub sample: u8,
}

impl LoadFileUtil for Test {}

const TEST: &str = r#"
title = 'TOML Example'
sample = 100
"#;

#[test]
fn main() {
    let test = Test::load_from_file_and_merge(
        TEST,
        toml::from_str,
        |origin, extra| {
            let merge = serde_toml_merge::merge(toml::from_str(origin)?, toml::from_str(extra)?)
                .map_err(|e| e.to_string())?;
            merge.try_into().map_err(Into::into)
        },
        "./tests/sample.toml",
    )
    .unwrap();

    assert_eq!(
        test,
        Test {
            title: "Overridden Title".to_owned(),
            sample: 100,
        }
    )
}
