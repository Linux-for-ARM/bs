//! Main error types

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("data deserialization error ({0}")]
    ParsingError(String),

    #[error("error reading file \"{0}\" ({1})")]
    ReadingError(String, String),

    #[error("error writing to file \"{0}\" ({1})")]
    WritingError(String, String),

    #[error("data serialization error ({0})")]
    SerializationError(String),

    #[error("file download error ({0})")]
    DownloadError(String),

    #[error("archive extract error ({0})")]
    ExtractError(String),
}