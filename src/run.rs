//! Running programs and commands

use anyhow::Result;
// use std::fs;
// use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

use serde::Deserialize;
use serde::Serialize;
use crate::traits::TomlConfig;
// use toml;

pub struct Run {
    /// The user on whose behalf the script will be run
    user: String,

    /// Path to the script to be run
    script: String,
}

#[derive(Deserialize, Serialize)]
pub struct RunResult {
    /// Script return code
    pub code: i32,

    /// Script run time (in seconds)
    pub time: u64,
}

#[derive(Deserialize, Serialize)]
pub struct RunLog {
    pub package: Vec<Log>,
}

#[derive(Deserialize, Serialize)]
pub struct Log {
    pub script: String,
    pub result: RunResult,
}

impl TomlConfig for RunLog {}

impl RunLog {
    pub fn append(&mut self, log: Log) -> Result<()> {
        self.package.push(log);
        self.write("build.log")?;

        Ok(())
    }
}

impl Run {
    pub fn new(user: &str, script: &str) -> Self {
        Self {
            user: user.to_string(),
            script: script.to_string(),
        }
    }

    pub fn run(&self) -> Result<RunResult> {
        let start_time = SystemTime::now();
        let code = Command::new("su")
            .arg(&self.user)
            .arg("-c")
            .arg(&self.script)
            .status()?
            .code()
            .unwrap_or(0);

        let time = start_time.elapsed()?.as_secs();

        Ok(RunResult { code, time })
    }
}
