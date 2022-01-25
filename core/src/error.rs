use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("HTTP client error: `{0}`")]
    HttpClientError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
