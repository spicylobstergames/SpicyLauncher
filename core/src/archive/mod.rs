pub mod gz;
pub mod zip;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ArchiveFormat {
    Gz,
    Zip,
}

impl fmt::Display for ArchiveFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl ArchiveFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        for format in Self::variants() {
            if path.extension().and_then(|v| v.to_str()) == Some(&format.to_string()) {
                return Some(*format);
            }
        }
        None
    }

    pub fn variants() -> &'static [Self] {
        &[Self::Gz, Self::Zip]
    }
}
