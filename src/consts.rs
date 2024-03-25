//! Constants and static variables

/// Path to the build log file
///
/// The build log file contains a list of already built packages; the
/// path to the build scripts, the build time in seconds, and the
/// returned exit code.
pub const BUILD_LOG: &str = "logs/build.log.toml";

/// Path to the file with settings shared to all build sessions
/// 
/// This file contains the name and version of the system being built
/// by LFA, the name of the builder of that system, and a number of
/// global environment variables that will be passed to the build
/// scripts.
pub const SHARED_CONF: &str = "assests/shared.toml";

/// Order of execution of build scripts
pub const BUILD_CONF: &str = "data/scripts/build.toml";

/// List of supported motherboards for which LFA build is possible
pub const MBOARDS_LST: &str = "data/mboards.toml";

/// List of LFA versions that can be built based on the
pub const LFA_VERSION_CONF: &str = "lfa_versions.toml";

/// URL to archives with build scripts
pub const BUILD_SCRIPTS_URL: &str = "https://github.com/Linux-for-ARM/build-scripts/";