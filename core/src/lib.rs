use std::str::FromStr;

use serde::Deserialize;

pub mod archive;
pub mod constant;
pub mod error;
pub mod github;
mod http;
pub mod release;
pub mod storage;
pub mod tracker;

/// The different games supported by the launcher
#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Game {
    Jumpy,
    Punchy,
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid game ID")]
pub struct InvalidGameId;

impl FromStr for Game {
    type Err = InvalidGameId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jumpy" => Ok(Game::Jumpy),
            "punchy" => Ok(Game::Punchy),
            _ => Err(InvalidGameId),
        }
    }
}

impl Game {
    pub fn id(&self) -> &'static str {
        match self {
            Game::Jumpy => "jumpy",
            Game::Punchy => "punchy",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Game::Jumpy => "Jumpy",
            Game::Punchy => "Punchy",
        }
    }

    pub fn binary_name(&self) -> String {
        let id = self.id();

        #[cfg(target_os = "windows")]
        let ext = ".exe";
        #[cfg(not(target_os = "windows"))]
        let ext = "";

        format!("{id}{ext}")
    }

    pub fn list() -> &'static [Game] {
        &[Game::Jumpy, Game::Punchy]
    }
}
