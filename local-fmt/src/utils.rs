use std::{error::Error, io::Read, path::Path};

type BoxError = Box<dyn Error + Send + Sync>;

pub trait LoadFileUtil: serde::de::DeserializeOwned {
    fn load_from_file<P, E>(
        deserializer: fn(&str) -> Result<Self, E>,
        path: P,
    ) -> Result<Self, BoxError>
    where
        P: AsRef<Path>,
        E: Into<BoxError>,
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

    fn load_from_file_or_init<P>(
        deserializer: fn(&str) -> Result<Self, BoxError>,
        seraializer: fn(&Self) -> Result<String, BoxError>,
        path: P,
    ) -> Result<Self, BoxError>
    where
        P: AsRef<Path>,
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

    fn load_from_file_and_merge<P, E>(
        value: &str,
        deserializer: fn(&str) -> Result<Self, E>,
        merge: fn(&str, &str) -> Result<Self, BoxError>,
        path: P,
    ) -> Result<Self, BoxError>
    where
        P: AsRef<Path>,
        E: Into<BoxError>,
        Self: Sized,
    {
        let path = path.as_ref();
        if !std::fs::exists(path)? {
            return deserializer(value).map_err(Into::into);
        };
        let mut file = std::fs::File::open(path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        merge(value, &text).map_err(Into::into)
    }
}
