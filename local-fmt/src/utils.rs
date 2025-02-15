use std::{error::Error, io::Read, path::Path};

pub trait MessagesUtil: serde::de::DeserializeOwned {
    fn load_from_file<P, E>(
        deserializer: fn(&str) -> Result<Self, E>,
        path: P,
    ) -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        P: AsRef<Path>,
        E: Error + Send + Sync + 'static,
        Self: Sized,
    {
        let path = path.as_ref();
        if !std::fs::exists(path)? {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("file not found: {:?}", path),
            )));
        };
        let mut file = std::fs::File::open(path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        deserializer(&text).map_err(Into::into)
    }

    fn load_from_file_or_init<P, E1, E2>(
        deserializer: fn(&str) -> Result<Self, E1>,
        seraializer: fn(&Self) -> Result<String, E2>,
        path: P,
    ) -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        P: AsRef<Path>,
        E1: Error + Send + Sync + 'static,
        E2: Error + Send + Sync + 'static,
        Self: Sized + Default + serde::Serialize,
    {
        let path = path.as_ref();
        if !std::fs::exists(path)? {
            let default = Self::default();
            let text = seraializer(&default)?;
            std::fs::write(path, text)?;
            return Ok(default);
        };
        let mut file = std::fs::File::open(path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        deserializer(&text).map_err(Into::into)
    }
}
