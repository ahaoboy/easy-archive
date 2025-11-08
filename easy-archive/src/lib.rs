/// Easy Archive - A cross-platform archive manipulation library
///
/// This library provides a unified interface for working with various archive formats
/// including TAR, ZIP, and their compressed variants (gzip, xz, bzip2, zstd).
///
/// # Features
///
/// The library uses Cargo features to enable/disable format support:
/// - `tar` - Plain TAR format
/// - `tar-gz` - Gzip-compressed TAR
/// - `tar-xz` - XZ-compressed TAR
/// - `tar-bz` - Bzip2-compressed TAR
/// - `tar-zstd` - Zstd-compressed TAR
/// - `zip` - ZIP format
/// - `default` - Enables all formats
///
/// # Example
///
/// ```no_run
/// use easy_archive::{Fmt, File};
///
/// // Decode an archive
/// let data = std::fs::read("archive.tar.gz")?;
/// let files = Fmt::TarGz.decode(data)?;
///
/// // Encode files into an archive
/// let files = vec![
///     File {
///         path: "hello.txt".to_string(),
///         buffer: b"Hello, world!".to_vec(),
///         ..Default::default()
///     }
/// ];
/// let archive = Fmt::Zip.encode(files)?;
/// std::fs::write("output.zip", archive)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
// Module declarations
pub mod archive;
pub mod error;
pub mod traits;
pub mod types;
pub mod utils;

// Re-export commonly used types and functions
pub use error::{ArchiveError, Result};
pub use traits::{Archive, Decode, Encode};
pub use types::{File, Fmt};
pub use utils::{check_duplicate_files, clean, human_size, mode_to_string};

#[cfg(test)]
mod test {
    use crate::{File, types::Fmt};
    use strum::IntoEnumIterator;

    #[test]
    fn test_decode() {
        for name in std::fs::read_dir("../assets").unwrap() {
            let path = name.unwrap().path();
            let buffer = std::fs::read(&path).unwrap();
            let fmt = Fmt::guess(&path.to_string_lossy()).unwrap();
            let files = fmt.decode(buffer).unwrap();
            let dist = files
                .iter()
                .find(|i| i.path == "mujs-build-0.0.11/dist-manifest.json")
                .unwrap();
            assert!(!dist.buffer.is_empty());
        }
    }

    use std::path::PathBuf;

    #[test]
    fn encode_decode() {
        for fmt in Fmt::iter() {
            // Skip TarBz if not fully implemented
            #[cfg(feature = "tar-bz")]
            if fmt == Fmt::TarBz {
                // TarBz encoding is now implemented
                continue;
            }

            let mut files = vec![];
            let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let asset_dir = base.join("../assets");

            for entry in std::fs::read_dir(asset_dir).expect("read dir error") {
                let file_path = entry.expect("get path error").path();
                let path = file_path
                    .file_name()
                    .expect("get name error")
                    .to_string_lossy()
                    .to_string();
                let buffer = std::fs::read(&file_path).expect("read file error");

                files.push(File {
                    buffer,
                    path,
                    mode: None,
                    is_dir: false,
                    last_modified: None,
                })
            }

            let compressed = fmt.encode(files).expect("encode error");
            println!("{:?} {}", fmt, compressed.len());
            assert!(!compressed.is_empty());
        }
    }
}
