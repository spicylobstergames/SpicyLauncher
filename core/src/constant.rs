pub const PROJECT_NAME: &str = "Fish Fight: Jumpy";

#[cfg(target_os = "windows")]
pub const BINARY_NAME: &str = "jumpy.exe";

#[cfg(not(target_os = "windows"))]
pub const BINARY_NAME: &str = "jumpy";

pub const DATA_DIR: &str = ".spicylobster";

pub const TEMP_DOWNLOAD_DIR: &str = "spicylobster-downloads";

pub const ARCHIVE_PREFIX: &str = "jumpy-";
