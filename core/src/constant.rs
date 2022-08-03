pub const PROJECT_NAME: &str = "Fish Fight: Jumpy";

#[cfg(target_os = "windows")]
pub const BINARY_NAME: &str = "jumpy.exe";

#[cfg(not(target_os = "windows"))]
pub const BINARY_NAME: &str = "jumpy";

pub const DATA_DIR: &str = ".spicylauncher";

pub const TEMP_DOWNLOAD_DIR: &str = "spicylauncher-downloads";

pub const ARCHIVE_PREFIX: &str = "jumpy-";
