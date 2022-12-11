use std::path::{PathBuf, StripPrefixError};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("HTTP client error: `{0}`")]
    HttpClient(#[from] reqwest::Error),
    #[error("HTTP error: `{0}`")]
    Http(String),
    #[error("Platform error: `{0}`")]
    Platform(String),
    #[error("Storage error: `{0}`")]
    Storage(String),
    #[error("Zip extract error: `{0}`")]
    Zip(#[from] zip::result::ZipError),
    #[error("Verify error: `{0}`")]
    Verify(String),
    #[error("UTF-8 error: `{0}`")]
    Utf8(String),
    #[error("Conversion error: `{0}`")]
    Conversion(#[from] std::num::TryFromIntError),
    #[error("Failed to strip toplevel directory {} from {}: {error}", .toplevel.to_string_lossy(), .path.to_string_lossy())]
    StripToplevel {
        toplevel: PathBuf,
        path: PathBuf,
        error: StripPrefixError,
    },
    #[error("Invalid game ID: `{0}`")]
    InvalidGameId(String),
}

pub type Result<T> = std::result::Result<T, Error>;
