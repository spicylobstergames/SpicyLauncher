pub const PROJECT_NAME: &str = "Fish Fight";

pub const GITHUB_REPOSITORY: &str = "fishfight/FishFight";

#[cfg(target_os = "windows")]
pub const BINARY_NAME: &str = "fishfight.exe";

#[cfg(not(target_os = "windows"))]
pub const BINARY_NAME: &str = "fishfight";

pub const DATA_DIR: &str = ".fishfight";

pub const TEMP_DOWNLOAD_DIR: &str = "fishfight-downloads";

pub const ARCHIVE_PREFIX: &str = "fishfight-";
