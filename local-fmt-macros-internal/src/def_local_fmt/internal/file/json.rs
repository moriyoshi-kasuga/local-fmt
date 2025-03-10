use super::MessageLoader;

pub struct JsonMessageLoader;

impl MessageLoader for JsonMessageLoader {
    const EXTENSION: &'static str = "json";

    type Value = serde_json::Value;
    type NestValue = serde_json::Map<String, serde_json::Value>;
    const NEST_VALUE_NAME: &'static str = "object";

    fn value_to_nest(value: Self::Value) -> Option<Self::NestValue> {
        match value {
            serde_json::Value::Object(s) => Some(s),
            _ => None,
        }
    }

    fn value_as_str(value: &Self::Value) -> Option<&str> {
        value.as_str()
    }

    fn value_from_str(content: &str) -> Result<Self::Value, String> {
        serde_json::from_str(content).map_err(|e| e.to_string())
    }

    fn iter_nested(value: Self::NestValue) -> impl Iterator<Item = (String, Self::Value)> {
        value.into_iter()
    }
}
