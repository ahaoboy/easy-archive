/// Error types for archive operations
use thiserror::Error;

/// Result type alias for archive operations
pub type Result<T> = std::result::Result<T, ArchiveError>;

/// Errors that can occur during archive operations
#[derive(Error, Debug)]
pub enum ArchiveError {
    /// I/O error occurred during archive operation
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to decode an archive
    #[error("Failed to decode {format} archive: {reason}")]
    DecodeFailed {
        /// The archive format that failed to decode
        format: String,
        /// The reason for the failure
        reason: String,
    },

    /// Failed to encode an archive
    #[error("Failed to encode {format} archive: {reason}")]
    EncodeFailed {
        /// The archive format that failed to encode
        format: String,
        /// The reason for the failure
        reason: String,
    },

    /// Duplicate file paths detected in the archive
    #[error("Duplicate file paths detected: {}", .paths.join(", "))]
    DuplicateFiles {
        /// List of duplicate file paths
        paths: Vec<String>,
    },

    /// The archive format is not supported or not enabled
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    /// The archive structure is invalid
    #[error("Invalid archive structure: {0}")]
    InvalidArchive(String),

    /// Compression operation failed
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Decompression operation failed
    #[error("Decompression error: {0}")]
    DecompressionError(String),
}
