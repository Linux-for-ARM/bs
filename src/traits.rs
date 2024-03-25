//! Basic traits for the data of the build system

use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use toml;

use crate::error::Error;
use crate::utils;

/// Trait for serializing/deserializing data to/from TOML-configs
pub trait TomlConfig {
    /// Reading TOML-config file and deserializing it into `Self` object
    fn parse<P>(pth: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let content = utils::read_to_string(&pth)?;
        Ok(toml::from_str(&content).map_err(|err| Error::ParsingError(err.to_string()))?)
    }

    /// Writing a serialized object `Self` to the TOML config file
    fn write<P>(&self, pth: P) -> Result<(), Error>
    where
        Self: Serialize,
        P: AsRef<Path>,
    {
        let data =
            toml::to_string(&self).map_err(|err| Error::SerializationError(err.to_string()))?;

        utils::write(&pth, data)?;

        Ok(())
    }
}
