#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageFormat {
    Text(String),
    StaticText(&'static str),
    Arg(usize),
}

/// N is argument length
#[derive(Debug, Clone)]
pub struct ConstMessage<const N: usize>(Vec<MessageFormat>);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConstMessageError {
    #[error("invalid number: {number} (please 0 <= number < {n})")]
    InvalidNumber { number: usize, n: usize },
    #[error("without number: {number} (not found in 0 <= number < {n})")]
    WithoutNumber { number: usize, n: usize },
}

impl<const N: usize> ConstMessage<N> {
    pub const fn const_check_and_panic(formats: &[MessageFormat]) -> &[MessageFormat] {
        match Self::const_check(formats) {
            Ok(ok) => ok,
            Err(err) => match err {
                ConstMessageError::InvalidNumber { .. } => {
                    panic!("has invalid number arg")
                }
                ConstMessageError::WithoutNumber { .. } => {
                    panic!("has without number arg")
                }
            },
        }
    }

    pub const fn const_check(
        formats: &[MessageFormat],
    ) -> Result<&[MessageFormat], ConstMessageError> {
        let mut numbers = [false; N];

        let mut current = 0;

        while formats.len() > current {
            if let MessageFormat::Arg(n) = formats[current] {
                if n >= N {
                    return Err(ConstMessageError::InvalidNumber { number: n, n: N });
                }
                numbers[n] = true;
            }
            current += 1;
        }

        let mut current = 0;

        while numbers.len() > current {
            if !numbers[current] {
                return Err(ConstMessageError::WithoutNumber {
                    number: current,
                    n: N,
                });
            }
            current += 1;
        }

        Ok(formats)
    }

    pub fn new(formats: Vec<MessageFormat>) -> Result<Self, ConstMessageError> {
        Self::const_check(&formats)?;

        Ok(Self(formats))
    }

    /// # Safety
    /// fill arg number by `[0, N)`
    pub const unsafe fn new_unchecked(formats: Vec<MessageFormat>) -> Self {
        Self(formats)
    }

    pub const fn args_len(&self) -> usize {
        N
    }

    pub fn format(&self, args: &[&str; N]) -> String {
        let mut text = String::new();

        for i in &self.0 {
            match i {
                MessageFormat::Text(s) => text.push_str(s),
                MessageFormat::Arg(n) => text.push_str(args[*n]),
                MessageFormat::StaticText(s) => text.push_str(s),
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

        Self::new(formats).map_err(serde::de::Error::custom)
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
                if v > usize::MAX as u64 {
                    return Err(serde::de::Error::custom(format!(
                        "please number between 0 and {}",
                        usize::MAX
                    )));
                }
                Ok(MessageFormat::Arg(v as usize))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v < 0 {
                    return Err(serde::de::Error::custom("please positive number"));
                }
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
            MessageFormat::StaticText(text) => serializer.serialize_str(text),
        }
    }
}
