use crate::error::{Error, Result};
use crate::release::{ArchiveFormat, Asset};
use flate2::read::GzDecoder;
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tar::Archive;

const DATA_DIR: &str = ".fishfight";
const TEMP_DOWNLOAD_DIR: &str = "fishfight-downloads";

#[derive(Debug)]
pub struct LocalStorage {
    pub temp: PathBuf,
    data: PathBuf,
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
        Ok(Self {
            temp: temp_dir,
            data: data_dir,
        })
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
                archive.unpack(&self.data.join(target_dir))?;
            }
            Some(ArchiveFormat::Zip) => {
                let zip = File::open(archive_path)?;
                zip_extract::extract(zip, &self.data.join(target_dir), true)?;
            }
            _ => {}
        }
        Ok(())
    }
}
