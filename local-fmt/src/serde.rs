use crate::ConstMessage;
use std::str::FromStr;

impl<'de, const N: usize> serde::Deserialize<'de> for ConstMessage<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;

        Self::from_str(&text).map_err(serde::de::Error::custom)
    }
}

impl<const N: usize> serde::Serialize for ConstMessage<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
