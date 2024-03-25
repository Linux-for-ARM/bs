//! List of supported motherboards

use serde::Deserialize;
use serde::Serialize;

use crate::consts::MBOARDS_LST;
use crate::error::Error;
use crate::traits::TomlConfig;

/// List of supported motherboards and it's configs
#[derive(Serialize, Deserialize)]
pub struct MBoards {
    pub board: Vec<MBoard>,
}

/// Specific motherboard configuration
#[derive(Serialize, Deserialize)]
pub struct MBoard {
    /// Motherboard ID
    /// 
    /// Used in the name of the directory with a number of files
    /// specific to that board and in a number of other places.
    pub name: String,

    /// Common (pretty) board name that can be displayed, for
    /// exapmle, in a pseudo-graphic configurator
    pub pretty: String,

    /// List of kernels supported by this motherboard
    pub kernel: Vec<Kernel>,
}

/// Linux kernel supported by a specific motherboard
#[derive(Serialize, Deserialize)]
pub struct Kernel {
    /// Kernel version
    pub version: String,

    /// Name of this kernel version, displayed in the
    /// pseudo-graphic configurator
    pub pretty: String,
}

impl TomlConfig for MBoards {}
impl TomlConfig for MBoard {}
impl TomlConfig for Kernel {}

impl MBoards {
    /// Reads the standard list of motherboards. Wrapper over [`crate::traits::TomlConfig::parse()`].
    pub fn read() -> Result<Self, Error> {
        MBoards::parse(MBOARDS_LST)
    }
}