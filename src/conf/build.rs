//! LFA software build information

use serde::Serialize;
use serde::Deserialize;

use crate::traits::TomlConfig;
use crate::error::Error;

use crate::conf::mboards::MBoard;
use crate::conf::conf::SharedConf;

// WARN: test version!!!
/// Master build system config
#[derive(Serialize, Deserialize)]
pub struct MasterConf {
    pub mboard: MBoard,
    pub settings: SharedConf,
}

impl TomlConfig for MasterConf {}
