use crate::archive::{self, ArchiveFormat};
use crate::constant::*;
use crate::error::{Error, Result};
use crate::release::{Asset, Release};
use crate::tracker::ProgressTracker;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

    pub fn extract_archive<Tracker: ProgressTracker>(
        &self,
        asset: &Asset,
        archive_path: &Path,
        release_version: &str,
        tracker: &mut Tracker,
    ) -> Result<()> {
        match asset.archive_format {
            Some(ArchiveFormat::Gz) => {
                archive::gz::extract(archive_path, &self.data_dir, release_version, tracker)?;
            }
            Some(ArchiveFormat::Zip) => {
                archive::zip::extract(
                    archive_path,
                    &self.data_dir.join(release_version),
                    true,
                    tracker,
                )?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_available_releases(&self) -> Result<Vec<Release>> {
        Ok(fs::read_dir(&self.data_dir)?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .filter(|entry| entry.is_dir() && entry.join(BINARY_NAME).exists())
            .filter_map(|directory| {
                directory
                    .file_name()
                    .map(|v| v.to_string_lossy().to_string())
            })
            .map(Release::from)
            .collect())
    }

    pub fn launch_game(&self, version: &str) -> Result<()> {
        let binary_path = &self.data_dir.join(&version).join(BINARY_NAME);
        log::debug!("Launching: {:?}", binary_path);
        Command::new(
            binary_path
                .to_str()
                .ok_or_else(|| Error::Utf8(String::from("path contains invalid characters")))?,
        )
        .current_dir(self.data_dir.join(&version))
        .spawn()?;
        Ok(())
    }
}
