/// Traits for archive encoding and decoding operations
use crate::{File, error::Result};

/// Trait for decoding archives from bytes
///
/// Implementors of this trait can decode archive data into a list of files.
///
/// This trait is only available when the `decode` feature is enabled.
#[cfg(feature = "decode")]
pub trait Decode {
    /// Decode an archive from a byte buffer
    ///
    /// # Arguments
    /// * `buffer` - The archive data (can be any type that converts to &[u8])
    ///
    /// # Returns
    /// * `Ok(Vec<File>)` - The extracted files on success
    /// * `Err(ArchiveError)` - If decoding fails
    ///
    /// # Example
    /// ```no_run
    /// use easy_archive::{Decode, archive::tar::Tar};
    ///
    /// let data = std::fs::read("archive.tar")?;
    /// let files = Tar::decode(data)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>>;
}

/// Trait for encoding files into archives
///
/// Implementors of this trait can encode a list of files into archive format.
///
/// This trait is only available when the `encode` feature is enabled.
#[cfg(feature = "encode")]
pub trait Encode {
    /// Encode files into an archive
    ///
    /// # Arguments
    /// * `files` - The list of files to include in the archive
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - The encoded archive data on success
    /// * `Err(ArchiveError)` - If encoding fails or duplicate files are detected
    ///
    /// # Example
    /// ```no_run
    /// use easy_archive::{Encode, File, archive::tar::Tar};
    ///
    /// let files = vec![
    ///     File {
    ///         path: "hello.txt".to_string(),
    ///         buffer: b"Hello, world!".to_vec(),
    ///         ..Default::default()
    ///     }
    /// ];
    /// let archive = Tar::encode(files)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn encode(files: Vec<File>) -> Result<Vec<u8>>;
}

/// Combined trait for types that support both encoding and decoding
///
/// This is a marker trait that indicates a type can both encode and decode archives.
/// Only available when both `encode` and `decode` features are enabled.
#[cfg(all(feature = "encode", feature = "decode"))]
pub trait Archive: Encode + Decode {}
