use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unsupported file type: {0}")]
    UnsupportedFile(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("empty dataset")]
    EmptyDataset,
    #[error("baseline error: {0}")]
    Baseline(String),
    #[error("rule error: {0}")]
    Rule(String),
    #[error("profile error: {0}")]
    Profile(String),
    #[error("report error: {0}")]
    Report(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<polars::error::PolarsError> for Error {
    fn from(err: polars::error::PolarsError) -> Self {
        Error::Io(err.to_string())
    }
}
