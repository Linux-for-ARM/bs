//! Serialization and deserialization of data and configurations

use serde::Deserialize;
use serde::Serialize;

use crate::traits::TomlConfig;

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

/// Local build system settings for a specific motherboard
#[derive(Deserialize, Serialize)]
pub struct MBoardSettings {
    /// Information about the Linux kernels available for this motherboard
    pub kernel: Vec<MBoardKernel>,
}

/// Information about the Linux kernels available for this motherboard
#[derive(Deserialize, Serialize)]
pub struct MBoardKernel {
    /// Name of the archive with the kernel source code
    pub archive: String,

    /// Name of the configuration file for this kernel version
    pub config: String,
}

/// Default build system settings
#[derive(Deserialize, Serialize)]
pub struct Defaults {
    pub system_name: String,
    pub system_version: String,
}

/// Build system config parameters (`.config.toml`)
#[derive(Deserialize, Serialize)]
pub struct DotConfig {
    /// Motherboard name (`MBoard.name`)
    pub board_name: String,

    /// Linux kernel version used
    pub kernel: MBoardKernel,

    /// System name and version
    pub system: Defaults,
}

impl TomlConfig for MBoardsAll {}
impl TomlConfig for MBoard {}
impl TomlConfig for MBoardSettings {}
impl TomlConfig for MBoardKernel {}
impl TomlConfig for Defaults {}
impl TomlConfig for DotConfig {}
