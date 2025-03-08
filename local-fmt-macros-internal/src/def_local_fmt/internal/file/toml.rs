use super::MessageLoader;

pub(crate) struct TomlMessageLoader;

impl MessageLoader for TomlMessageLoader {
    const EXTENSION: &'static str = "toml";

    type Value = toml::Value;
    type NestValue = toml::Table;
    const NEST_VALUE_NAME: &'static str = "table";

    fn value_to_nest(value: Self::Value) -> Option<Self::NestValue> {
        match value {
            toml::Value::Table(s) => Some(s),
            _ => None,
        }
    }

    fn value_as_str(value: &Self::Value) -> Option<&str> {
        value.as_str()
    }

    fn value_from_str(content: &str) -> Result<Self::Value, String> {
        toml::from_str(content).map_err(|e| e.to_string())
    }

    fn iter_nested(value: Self::NestValue) -> impl Iterator<Item = (String, Self::Value)> {
        value.into_iter()
    }
}
