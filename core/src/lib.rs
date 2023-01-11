use serde::Deserialize;
use std::str::FromStr;

pub mod archive;
pub mod constant;
pub mod error;
pub mod github;
mod http;
pub mod release;
pub mod storage;
pub mod tracker;

/// The different games supported by the launcher.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Game {
    Jumpy,
    Punchy,
    Thetawave,
}

impl FromStr for Game {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jumpy" => Ok(Game::Jumpy),
            "punchy" => Ok(Game::Punchy),
            "thetawave" => Ok(Game::Thetawave),
            id => Err(error::Error::InvalidGameId(id.to_string())),
        }
    }
}

impl Game {
    pub fn id(&self) -> &'static str {
        match self {
            Game::Jumpy => "jumpy",
            Game::Punchy => "punchy",
            Game::Thetawave => "thetawave",
        }
    }

    pub fn binary_name(&self) -> String {
        let id = self.id();
        if cfg!(target_os = "windows") {
            format!("{}.exe", id)
        } else {
            id.to_string()
        }
    }

    pub fn list() -> &'static [Game] {
        &[Game::Jumpy, Game::Punchy, Game::Thetawave]
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Game::Jumpy => "Jumpy",
            Game::Punchy => "Punchy",
            Game::Thetawave => "Thetawave",
        };
        write!(f, "{}", name)
    }
}
