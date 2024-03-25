//! Information about the LFA versions available for building

use serde::Deserialize;
use serde::Serialize;

use std::process::Command;

use crate::consts::BUILD_SCRIPTS_URL;
use crate::consts::LFA_VERSION_CONF;
use crate::error::Error;
use crate::traits::TomlConfig;
use crate::utils::download;
use crate::utils::extract;

/// List of all LFA versions
#[derive(Serialize, Deserialize)]
pub struct LFAVersion {
    pub version: Vec<Version>,
}

/// Information on the specific LFA version
#[derive(Serialize, Deserialize)]
pub struct Version {
    /// LFA version
    pub version: String,

    /// Indicates if this a stable or test version
    pub is_stable: bool,
}

impl TomlConfig for LFAVersion {}
impl TomlConfig for Version {}

impl LFAVersion {
    /// Reads the standard LFA version list. Wrapper over [`crate::traits::TomlConfig::parse()`].
    pub fn read() -> Result<Self, Error> {
        LFAVersion::parse(LFA_VERSION_CONF)
    }

    /// Downloads an archive with files of the build system
    ///
    /// The archive contains build instructions and a number of other files. Each
    /// archive refers to a specific LFA version.
    pub fn download(&self, ver: &Version) -> Result<String, Error> {
        let url = format!("{}/scripts-{}.tar.xz", BUILD_SCRIPTS_URL, &ver.version);
        download(&url)
    }

    /// Extracts the downloaded archive
    ///
    /// It is guaranteed that the archive contains `data/` directory
    /// with all necessary data.
    pub fn extract(&self, ver: &Version) -> Result<(), Error> {
        let file = format!("./scripts-{}.tar.xz", &ver.version);
        extract(&file)
    }
}
