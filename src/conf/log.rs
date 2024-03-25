//! Recording information about the build progress to a log file

use serde::Deserialize;
use serde::Serialize;

use crate::consts::BUILD_LOG;
use crate::error::Error;
use crate::traits::TomlConfig;

/// List of already built packages
#[derive(Serialize, Deserialize)]
pub struct BuildLog {
    /// Information about an already built package
    pub script: Vec<ScriptInfo>,
}

/// Information about an already built package
#[derive(Serialize, Deserialize)]
pub struct ScriptInfo {
    /// Script name (e.g. `data/scripts/cross-compiler/binutils.sh`)
    pub name: String,

    /// Run time (sec.)
    pub build_time: u64,

    /// Return code
    pub exit_code: i32,
}

impl TomlConfig for BuildLog {}
impl TomlConfig for ScriptInfo {}

impl BuildLog {
    /// Reads the standard log file. Wrapper over [`crate::traits::TomlConfig::parse()`].
    pub fn read() -> Result<Self, Error> {
        BuildLog::parse(BUILD_LOG)
    }

    /// Adds a record of the built package to the log file
    pub fn append(mut self, script: ScriptInfo) -> Result<Self, Error> {
        self.script.push(script);
        self.write(BUILD_LOG)?;

        Ok(self)
    }
}
