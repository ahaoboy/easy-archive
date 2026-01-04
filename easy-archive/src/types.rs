/// Core types for archive operations
pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::error::Result;

#[cfg(feature = "tar")]
use crate::archive::Tar;
#[cfg(feature = "tar-bz")]
use crate::archive::TarBz;
#[cfg(feature = "tar-gz")]
use crate::archive::TarGz;
#[cfg(feature = "tar-xz")]
use crate::archive::TarXz;
#[cfg(feature = "tar-zstd")]
use crate::archive::TarZstd;
#[cfg(feature = "zip")]
use crate::archive::Zip;
#[cfg(feature = "7z")]
use crate::archive::seven_zip::SevenZip;

#[cfg(feature = "decode")]
use crate::traits::Decode;
#[cfg(feature = "encode")]
use crate::traits::Encode;

/// Archive format enumeration
///
/// Represents the supported archive formats. Each variant is conditionally
/// compiled based on the corresponding feature flag.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fmt {
    /// Plain tar archive format
    #[cfg(feature = "tar")]
    Tar,
    /// Gzip-compressed tar archive (.tar.gz, .tgz)
    #[cfg(feature = "tar-gz")]
    TarGz,
    /// XZ-compressed tar archive (.tar.xz, .txz)
    #[cfg(feature = "tar-xz")]
    TarXz,
    /// Bzip2-compressed tar archive (.tar.bz2, .tbz2)
    #[cfg(feature = "tar-bz")]
    TarBz,
    /// Zstd-compressed tar archive (.tar.zst, .tzst, .tzstd)
    #[cfg(feature = "tar-zstd")]
    TarZstd,
    /// ZIP archive format
    #[cfg(feature = "zip")]
    Zip,
    /// 7z archive format
    #[cfg(feature = "7z")]
    SevenZip,
}

impl Fmt {
    /// Decode an archive from bytes
    ///
    /// # Arguments
    /// * `buffer` - The archive data as bytes
    ///
    /// # Returns
    /// * `Ok(Vec<File>)` - List of files extracted from the archive
    /// * `Err(ArchiveError)` - If decoding fails
    ///
    /// # Example
    /// ```no_run
    /// use easy_archive::Fmt;
    /// let data = std::fs::read("archive.tar.gz")?;
    /// let files = Fmt::TarGz.decode(data)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg(feature = "decode")]
    pub fn decode(&self, buffer: Vec<u8>) -> Result<Vec<File>> {
        match self {
            #[cfg(feature = "zip")]
            Fmt::Zip => Zip::decode(buffer),
            #[cfg(feature = "tar")]
            Fmt::Tar => Tar::decode(buffer),
            #[cfg(feature = "tar-gz")]
            Fmt::TarGz => TarGz::decode(buffer),
            #[cfg(feature = "tar-xz")]
            Fmt::TarXz => TarXz::decode(buffer),
            #[cfg(feature = "tar-bz")]
            Fmt::TarBz => TarBz::decode(buffer),
            #[cfg(feature = "tar-zstd")]
            Fmt::TarZstd => TarZstd::decode(buffer),
            #[cfg(feature = "7z")]
            Fmt::SevenZip => SevenZip::decode(buffer),
        }
    }

    /// Encode files into an archive
    ///
    /// # Arguments
    /// * `files` - List of files to include in the archive
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - The encoded archive as bytes
    /// * `Err(ArchiveError)` - If encoding fails or duplicate files are detected
    ///
    /// # Example
    /// ```no_run
    /// use easy_archive::{Fmt, File};
    /// let files = vec![
    ///     File {
    ///         path: "hello.txt".to_string(),
    ///         buffer: b"Hello, world!".to_vec(),
    ///         ..Default::default()
    ///     }
    /// ];
    /// let archive = Fmt::TarGz.encode(files)?;
    /// std::fs::write("archive.tar.gz", archive)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg(feature = "encode")]
    pub fn encode(&self, files: Vec<File>) -> Result<Vec<u8>> {
        match self {
            #[cfg(feature = "zip")]
            Fmt::Zip => Zip::encode(files),
            #[cfg(feature = "tar")]
            Fmt::Tar => Tar::encode(files),
            #[cfg(feature = "tar-gz")]
            Fmt::TarGz => TarGz::encode(files),
            #[cfg(feature = "tar-xz")]
            Fmt::TarXz => TarXz::encode(files),
            #[cfg(feature = "tar-bz")]
            Fmt::TarBz => TarBz::encode(files),
            #[cfg(feature = "tar-zstd")]
            Fmt::TarZstd => TarZstd::encode(files),
            #[cfg(feature = "7z")]
            Fmt::SevenZip => SevenZip::encode(files),
        }
    }

    /// Guess the archive format from a filename
    ///
    /// # Arguments
    /// * `name` - The filename to analyze
    ///
    /// # Returns
    /// * `Some(Fmt)` - The detected format
    /// * `None` - If the format cannot be determined
    ///
    /// # Example
    /// ```
    /// use easy_archive::Fmt;
    /// assert_eq!(Fmt::guess("archive.tar.gz"), Some(Fmt::TarGz));
    /// assert_eq!(Fmt::guess("file.zip"), Some(Fmt::Zip));
    /// assert_eq!(Fmt::guess("unknown.txt"), None);
    /// ```
    pub fn guess(name: &str) -> Option<Self> {
        for fmt in Fmt::iter() {
            for ext in fmt.extensions() {
                if name.ends_with(ext) {
                    return Some(fmt);
                }
            }
        }
        None
    }

    /// Get the file extensions for this format
    ///
    /// # Returns
    /// A slice of file extension strings (including the leading dot)
    ///
    /// # Example
    /// ```
    /// use easy_archive::Fmt;
    /// assert_eq!(Fmt::TarGz.extensions(), &[".tar.gz", ".tgz"]);
    /// ```
    pub fn extensions(&self) -> &[&'static str] {
        match self {
            #[cfg(feature = "tar")]
            Fmt::Tar => &[".tar"],
            #[cfg(feature = "tar-gz")]
            Fmt::TarGz => &[".tar.gz", ".tgz"],
            #[cfg(feature = "tar-xz")]
            Fmt::TarXz => &[".tar.xz", ".txz"],
            #[cfg(feature = "tar-bz")]
            Fmt::TarBz => &[".tar.bz2", ".tbz2", ".tbz"],
            #[cfg(feature = "tar-zstd")]
            Fmt::TarZstd => &[".tzstd", ".tzst", ".tar.zst"],
            #[cfg(feature = "zip")]
            Fmt::Zip => &[".zip"],
            #[cfg(feature = "7z")]
            Fmt::SevenZip => &[".7z"],
        }
    }
}

/// Represents a file or directory entry in an archive
///
/// This structure holds all the metadata and content for a single entry
/// in an archive file.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Default)]
pub struct File {
    /// The file content as raw bytes
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub buffer: Vec<u8>,

    /// The relative path of the file within the archive
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub path: String,

    /// Unix file permissions (e.g., 0o755 for rwxr-xr-x)
    pub mode: Option<u32>,

    /// Whether this entry represents a directory
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "isDir"))]
    pub is_dir: bool,

    /// Last modification time as Unix timestamp (seconds since epoch)
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "lastModified"))]
    pub last_modified: Option<u64>,
}

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl File {
    /// Create a new File entry
    ///
    /// # Arguments
    /// * `path` - The relative path within the archive
    /// * `buffer` - The file content
    /// * `mode` - Optional Unix permissions
    /// * `is_dir` - Whether this is a directory
    /// * `last_modified` - Optional modification timestamp
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(
        path: String,
        buffer: Vec<u8>,
        mode: Option<u32>,
        is_dir: bool,
        last_modified: Option<u64>,
    ) -> Self {
        File {
            path,
            buffer,
            mode,
            is_dir,
            last_modified,
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl File {
    /// Get the file buffer (WASM only)
    ///
    /// Note: This consumes the File to reduce memory consumption
    #[wasm_bindgen(getter = buffer)]
    pub fn get_buffer(self) -> Vec<u8> {
        self.buffer
    }

    /// Set the file buffer (WASM only)
    #[wasm_bindgen(setter = buffer)]
    pub fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    /// Get the file path (WASM only)
    #[wasm_bindgen(getter = path)]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// Set the file path (WASM only)
    #[wasm_bindgen(setter = path)]
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    /// Get the buffer size in bytes (WASM only)
    #[wasm_bindgen(getter = bufferSize)]
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    /// Clone the File (WASM only)
    #[wasm_bindgen]
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Self {
        Clone::clone(self)
    }
}

#[cfg(test)]
mod test {
    use super::Fmt;

    #[test]
    fn test_guess() {
        let test_cases = vec![
            #[cfg(feature = "zip")]
            ("a.zip", Fmt::Zip),
            #[cfg(feature = "tar")]
            ("a.tar", Fmt::Tar),
            #[cfg(feature = "tar-gz")]
            ("a.tar.gz", Fmt::TarGz),
            #[cfg(feature = "tar-xz")]
            ("a.tar.xz", Fmt::TarXz),
            #[cfg(feature = "tar-bz")]
            ("a.tar.bz2", Fmt::TarBz),
        ];

        for (name, fmt) in test_cases {
            assert_eq!(
                Fmt::guess(name),
                Some(fmt),
                "Failed to guess format for {}",
                name
            );
        }
    }
}
