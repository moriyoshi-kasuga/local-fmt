use super::MessageLoader;

pub struct YamlMessageLoader;

impl MessageLoader for YamlMessageLoader {
    const EXTENSION: &'static str = "yaml";

    type Value = yaml_rust::Yaml;
    type NestValue = yaml_rust::yaml::Hash;
    const NEST_VALUE_NAME: &'static str = "hash";

    fn value_to_nest(value: Self::Value) -> Option<Self::NestValue> {
        value.into_hash()
    }

    fn value_as_str(value: &Self::Value) -> Option<&str> {
        value.as_str()
    }

    fn value_from_str(content: &str) -> Result<Self::Value, String> {
        yaml_rust::YamlLoader::load_from_str(content)
            .map_err(|v| v.to_string())
            .and_then(|mut docs| {
                docs.pop()
                    .ok_or_else(|| "yaml document is empty".to_string())
            })
    }

    fn iter_nested(value: Self::NestValue) -> impl Iterator<Item = (String, Self::Value)> {
        value.into_iter().map(|(k, v)| {
            let k = match k {
                yaml_rust::Yaml::String(s) => s,
                _ => panic!("Expected a string key, but got {:?}", k),
            };
            (k, v)
        })
    }
}
