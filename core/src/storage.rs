use crate::archive::{self, ArchiveFormat};
use crate::error::{Error, Result};
use crate::release::{Asset, Release};
use crate::tracker::ProgressTracker;
use crate::{constant::*, Game};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct LocalStorage {
    pub temp_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl LocalStorage {
    pub fn init() -> Result<Self> {
        let temp_dir = env::temp_dir().join(TEMP_DOWNLOAD_DIR);
        let data_dir = dirs_next::home_dir()
            .ok_or_else(|| Error::Storage(String::from("home directory not found")))?
            .join(DATA_DIR);
        for path in &[&temp_dir, &data_dir] {
            if !path.exists() {
                fs::create_dir(path)?;
            }
        }
        Ok(Self { temp_dir, data_dir })
    }

    /// Get the filesystem path that stores the versions for a specific game.
    pub fn game_path(&self, game: Game) -> PathBuf {
        self.data_dir.join(game.id())
    }

    /// Get the filesystem path storing the specified version of the game.
    ///
    /// > **Note:** The path may or may not exist.
    pub fn version_path(&self, game: Game, release_version: &str) -> PathBuf {
        self.game_path(game).join(release_version)
    }

    /// Remove the specified version from the filesystem, if it is installed.
    pub fn remove_version(&self, game: Game, release_version: &str) -> Result<()> {
        let target_dir = self.version_path(game, release_version);
        if target_dir.exists() {
            std::fs::remove_dir_all(target_dir)?;
        }
        Ok(())
    }

    pub fn extract_archive<Tracker: ProgressTracker>(
        &self,
        asset: &Asset,
        archive_path: &Path,
        game: Game,
        release_version: &str,
        tracker: &mut Tracker,
    ) -> Result<()> {
        match asset.archive_format {
            Some(ArchiveFormat::Gz) => {
                archive::gz::extract(
                    archive_path,
                    game,
                    &self.game_path(game),
                    release_version,
                    tracker,
                )?;
            }
            Some(ArchiveFormat::Zip) => {
                archive::zip::extract(
                    archive_path,
                    &self.version_path(game, release_version),
                    true,
                    tracker,
                )?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_available_releases(&self, game: Game) -> Result<Vec<Release>> {
        let game_path = self.game_path(game);

        if !game_path.exists() {
            return Ok(Vec::default());
        }

        Ok(fs::read_dir(game_path)?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .filter(|entry| entry.is_dir() && entry.join(game.binary_name()).exists())
            .filter_map(|directory| {
                directory
                    .file_name()
                    .map(|v| v.to_string_lossy().to_string())
            })
            .map(Release::from)
            .collect())
    }

    pub fn launch_game(&self, game: Game, version: &str) -> Result<()> {
        let binary_path = &self
            .data_dir
            .join(game.id())
            .join(version)
            .join(game.binary_name());
        log::debug!("Launching: {:?}", binary_path);
        Command::new(
            binary_path
                .to_str()
                .ok_or_else(|| Error::Utf8(String::from("path contains invalid characters")))?,
        )
        .current_dir(self.version_path(game, version))
        .spawn()?;
        Ok(())
    }
}
