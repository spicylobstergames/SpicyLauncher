use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] fish_launcher_core::error::Error),
    #[error("Version not found: `{0}`")]
    UnknownVersion(String),
}

pub type Result<T> = std::result::Result<T, Error>;
