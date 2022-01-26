use crate::error::{Error, Result};
use bytesize::ByteSize;
use platforms::platform::Platform;
use std::fmt;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub enum ArchiveFormat {
    Gz,
    Zip,
}

impl fmt::Display for ArchiveFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl ArchiveFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        for format in Self::variants() {
            if path.extension().and_then(|v| v.to_str()) == Some(&format.to_string()) {
                return Some(*format);
            }
        }
        None
    }

    pub fn variants() -> &'static [Self] {
        &[Self::Gz, Self::Zip]
    }
}

#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
    pub download_url: String,
    pub size: u64,
    pub archive_format: Option<ArchiveFormat>,
}

impl Asset {
    pub fn new(name: String, download_url: String, size: u64) -> Self {
        Self {
            archive_format: ArchiveFormat::from_path(Path::new(&name)),
            name,
            download_url,
            size,
        }
    }

    pub fn get_size(&self) -> String {
        ByteSize(self.size).to_string_as(false)
    }
}

#[derive(Debug)]
pub struct Release {
    pub name: String,
    pub version: String,
    pub assets: Vec<Asset>,
}

impl Release {
    pub fn get_asset(&self) -> Result<Asset> {
        let platform = Platform::guess_current()
            .ok_or_else(|| Error::Platform(String::from("unknown platform")))?;
        self.assets
            .clone()
            .into_iter()
            .find(|asset| {
                asset.archive_format.is_some() && asset.name.contains(platform.target_triple)
            })
            .ok_or_else(|| {
                Error::Platform(String::from(
                    "release assets for the current platform do not exist",
                ))
            })
    }
}
