//! Serialization and deserialization of data and configurations

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;
use toml;

/// List of motherboards (single board computers) for which system build is possible
#[derive(Deserialize, Serialize)]
pub struct MBoardsAll {
    /// List of motherboards (single board computers) for which system build is possible
    pub mboard: Vec<MBoard>,
}

/// List of motherboards (single board computers) for which system build is possible
#[derive(Deserialize, Serialize)]
pub struct MBoard {
    /// A short name that is used in directory names and other file names, and to
    /// specify the motherboard in the build system parameters (e.g. `opi-3lts`)
    pub name: String,

    /// The full name of the motherboard, which is used when viewing the list of
    /// compatible hardware and generating the motherboard selection menu (e.g.
    /// `Orange Pi 3 LTS`)
    pub pretty: String,
}

impl MBoardsAll {
    pub fn read<P: AsRef<Path>>(pth: P) -> Result<Self> {
        let data = fs::read_to_string(&pth)?;
        Ok(toml::from_str(&data)?)
    }

    pub(crate) fn write<P: AsRef<Path>>(&self, pth: P) -> Result<()> {
        let data = toml::to_string_pretty(&self)?;
        fs::write(&pth, data)?;

        Ok(())
    }
}
