#[cfg(feature = "macros")]
pub use local_fmt_macros as macros;

#[derive(Debug)]
pub enum MessageFormat {
    Text(String),
    Arg(usize),
}

/// N is argument length
#[derive(Debug)]
pub struct ConstMessage<const N: usize>(Vec<MessageFormat>);

impl<const N: usize> ConstMessage<N> {
    pub fn new(formats: Vec<MessageFormat>) -> Self {
        Self(formats)
    }

    pub fn format(&self, args: &[&str; N]) -> String {
        let mut text = String::new();

        for i in &self.0 {
            match i {
                MessageFormat::Text(s) => text.push_str(s),
                MessageFormat::Arg(n) => text.push_str(args[*n]),
            }
        }

        text
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for ConstMessage<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let formats = Vec::<MessageFormat>::deserialize(deserializer)?;
        for i in &formats {
            if let MessageFormat::Arg(n) = i {
                if *n >= N {
                    return Err(serde::de::Error::custom(format!(
                        "please set number between 0 and {} exclusive",
                        N
                    )));
                }
            }
        }

        if formats.len() != N {
            return Err(serde::de::Error::custom(format!(
                "please set {} arguments",
                N
            )));
        }

        Ok(Self(formats))
    }
}

impl<const N: usize> serde::Serialize for ConstMessage<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for MessageFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = MessageFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("allow string or number")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MessageFormat::Arg(v as usize))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MessageFormat::Text(v.to_string()))
            }
        }
        deserializer.deserialize_string(Visitor)
    }
}

impl serde::Serialize for MessageFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageFormat::Text(text) => serializer.serialize_str(text),
            MessageFormat::Arg(num) => serializer.serialize_u64(*num as u64),
        }
    }
}

#[macro_export]
macro_rules! gen_const_message {
     (@gen $text:literal) => {
         $crate::MessageFormat::Text($text.to_string())
     };
     (@gen {$number:literal}) => {
         $crate::MessageFormat::Arg($number)
     };
     ($($tt:tt),*) => {
         $crate::ConstMessage::new(vec![$(gen_const_message!(@gen $tt)),*])
     }
 }
