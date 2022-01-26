use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("HTTP client error: `{0}`")]
    HttpClient(#[from] reqwest::Error),
    #[error("Platform error: `{0}`")]
    Platform(String),
    #[error("Storage error: `{0}`")]
    Storage(String),
    #[error("Zip extract error: `{0}`")]
    Zip(#[from] zip_extract::ZipExtractError),
    #[error("Verify error: `{0}`")]
    Verify(String),
    #[error("UTF-8 error: `{0}`")]
    Utf8(String),
}

pub type Result<T> = std::result::Result<T, Error>;
