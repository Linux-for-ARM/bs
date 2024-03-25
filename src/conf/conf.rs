//! Basic configuration files of the build system

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::consts::BUILD_CONF;
use crate::consts::SHARED_CONF;
use crate::error::Error;
use crate::traits::TomlConfig;

/// Shared settings for all build sessions
#[derive(Serialize, Deserialize)]
pub struct SharedConf {
    /// Shared parameters
    pub settings: SettingsSharedConf,

    /// Shared environment variables
    pub env: EnvSharedConf,
}

/// Shared parameters
#[derive(Serialize, Deserialize)]
pub struct SettingsSharedConf {
    pub system_name: String,
    pub system_version: String,
    pub system_builder: String,
}

/// Shared environment variables
#[derive(Serialize, Deserialize)]
pub struct EnvSharedConf {
    pub env: HashMap<String, String>,
}

/// Order of building the necessary scripts
#[derive(Serialize, Deserialize)]
pub struct BuildConf {
    pub scripts: Vec<String>,
}

impl TomlConfig for SharedConf {}
impl TomlConfig for BuildConf {}

impl TomlConfig for SettingsSharedConf {}
impl TomlConfig for EnvSharedConf {}

impl SharedConf {
    /// Reads the standard shared config. Wrapper over [`crate::traits::TomlConfig::parse()`].
    pub fn read() -> Result<Self, Error> {
        SharedConf::parse(SHARED_CONF)
    }
}

impl BuildConf {
    /// Reads the standard build config. Wrapper over [`crate::traits::TomlConfig::parse()`].
    pub fn read() -> Result<Self, Error> {
        BuildConf::parse(BUILD_CONF)
    }
}

impl Default for SharedConf {
    fn default() -> Self {
        Self {
            settings: SettingsSharedConf::default(),
            env: EnvSharedConf::default(),
        }
    }
}

impl Default for SettingsSharedConf {
    fn default() -> Self {
        Self {
            system_name: "ALFA".to_string(),
            system_version: "1.0.0".to_string(),
            system_builder: "bs".to_string(),
        }
    }
}

impl Default for EnvSharedConf {
    fn default() -> Self {
        Self {
            env: HashMap::new(),
        }
    }
}
