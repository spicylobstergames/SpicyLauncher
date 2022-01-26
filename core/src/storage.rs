use crate::constant::*;
use crate::error::{Error, Result};
use crate::release::{ArchiveFormat, Asset};
use flate2::read::GzDecoder;
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;

#[derive(Debug)]
pub struct LocalStorage {
    pub temp_dir: PathBuf,
    data_dir: PathBuf,
}

impl LocalStorage {
    pub fn init() -> Result<Self> {
        let temp_dir = env::temp_dir().join(TEMP_DOWNLOAD_DIR);
        let data_dir = dirs_next::home_dir()
            .ok_or_else(|| Error::Storage(String::from("home directory not found")))?
            .join(DATA_DIR);
        for path in &[&temp_dir, &data_dir] {
            if !path.exists() {
                fs::create_dir(&path)?;
            }
        }
        Ok(Self { temp_dir, data_dir })
    }

    pub fn extract_archive(
        &self,
        asset: &Asset,
        archive_path: &Path,
        target_dir: &str,
    ) -> Result<()> {
        match asset.archive_format {
            Some(ArchiveFormat::Gz) => {
                let tar_gz = File::open(archive_path)?;
                let tar = GzDecoder::new(tar_gz);
                let mut archive = Archive::new(tar);
                archive.unpack(&self.data_dir.join(target_dir))?;
            }
            Some(ArchiveFormat::Zip) => {
                let zip = File::open(archive_path)?;
                zip_extract::extract(zip, &self.data_dir.join(target_dir), true)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_available_releases(&self) -> Result<Vec<String>> {
        Ok(fs::read_dir(&self.data_dir)?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .filter(|entry| entry.is_dir() && entry.join(BINARY_NAME).exists())
            .filter_map(|directory| {
                directory
                    .file_name()
                    .map(|v| v.to_string_lossy().to_string())
            })
            .collect())
    }

    pub fn launch_game(&self, version: &str) -> Result<()> {
        let binary_path = &self.data_dir.join(version).join(BINARY_NAME);
        Command::new(
            binary_path
                .to_str()
                .ok_or_else(|| Error::Utf8(String::from("path contains invalid characters")))?,
        )
        .current_dir(self.data_dir.join(version))
        .spawn()?
        .wait()?;
        Ok(())
    }
}
