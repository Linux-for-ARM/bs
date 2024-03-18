use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub work_mode: WorkMode,
}

#[derive(Subcommand)]
pub enum WorkMode {
    /// Set up the build system
    Menuconfig,

    /// Prepare the host system for LFA builds
    ConfigureHost,

    /// Perform the LFA build
    Build,
}
