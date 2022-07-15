use crate::archive::ArchiveFormat;
use crate::error::{Error, Result};
use bytesize::ByteSize;
use platforms::platform::Platform;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Release {
    pub name: String,
    pub version: String,
    pub body: String,
    pub installed: bool,
    #[serde(skip)]
    pub assets: Vec<Asset>,
}

impl From<String> for Release {
    fn from(version: String) -> Self {
        Self {
            version,
            ..Self::default()
        }
    }
}

impl Release {
    pub fn get_asset(&self) -> Result<Asset> {
        let platform = match env::var("PLATFORM_OVERRIDE").ok() {
            Some(target_triple) => Platform::find(&target_triple),
            None => guess_host_triple::guess_host_triple().and_then(Platform::find),
        }
        .ok_or_else(|| Error::Platform(String::from("unknown platform")))?;
        log::debug!("Platform: {}", platform);
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
