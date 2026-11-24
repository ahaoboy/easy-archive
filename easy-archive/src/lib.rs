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
/// - `gz` - Single-file gzip stream
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

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "convert")]
pub mod convert;

// Re-export commonly used types and functions
pub use error::{ArchiveError, Result};
pub use types::{File, Fmt};
pub use utils::{clean, human_size, mode_to_string};

#[cfg(feature = "decode")]
pub use traits::Decode;

#[cfg(feature = "encode")]
pub use traits::Encode;

#[cfg(feature = "encode")]
pub use utils::check_duplicate_files;

#[cfg(all(feature = "encode", feature = "decode"))]
pub use traits::Archive;

#[cfg(feature = "convert")]
pub use convert::{convert_archive, convert_archive_file};

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
            // Single-file formats (plain gzip) encode exactly one file, so
            // they are exercised by their own round-trip test instead.
            if fmt.is_single_file() {
                continue;
            }

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
                    link_target: None,
                    last_modified: None,
                })
            }

            let compressed = fmt.encode(files).expect("encode error");
            println!("{:?} {}", fmt, compressed.len());
            assert!(!compressed.is_empty());
        }
    }

    /// Formats that can store a symbolic link, i.e. the TAR family and ZIP.
    fn link_formats() -> Vec<Fmt> {
        Fmt::iter()
            .filter(|fmt| !fmt.is_single_file())
            .filter(|fmt| {
                #[cfg(feature = "7z")]
                if *fmt == Fmt::SevenZip {
                    // sevenz-rust2 exposes no link metadata; the encoder
                    // refuses links instead of writing an empty file.
                    return false;
                }
                // bzip2-rs has no encoder, so TarBz encoding is unsupported.
                #[cfg(feature = "tar-bz")]
                if *fmt == Fmt::TarBz {
                    return false;
                }
                true
            })
            .collect()
    }

    /// A symlink must survive encode → decode as a link, not as an empty file.
    #[test]
    fn symlink_round_trip() {
        for fmt in link_formats() {
            let files = vec![
                File {
                    path: "lib/libfoo.so.1.2.3".to_string(),
                    buffer: b"ELF".to_vec(),
                    mode: Some(0o755),
                    is_dir: false,
                    link_target: None,
                    last_modified: None,
                },
                File::symlink("lib/libfoo.so.1".to_string(), "libfoo.so.1.2.3".to_string()),
            ];

            let encoded = fmt
                .encode(files)
                .unwrap_or_else(|e| panic!("{fmt:?} failed to encode a symlink: {e}"));
            let decoded = fmt
                .decode(encoded)
                .unwrap_or_else(|e| panic!("{fmt:?} failed to decode a symlink: {e}"));

            let link = decoded
                .iter()
                .find(|file| file.path.ends_with("libfoo.so.1"))
                .unwrap_or_else(|| panic!("{fmt:?} lost the symlink: {decoded:?}"));
            assert_eq!(
                link.link_target.as_deref(),
                Some("libfoo.so.1.2.3"),
                "{fmt:?}"
            );
            assert!(link.is_symlink(), "{fmt:?}");
            assert!(link.buffer.is_empty(), "{fmt:?}");
            assert!(!link.is_dir, "{fmt:?}");

            // The link target itself must still be a regular file.
            let target = decoded
                .iter()
                .find(|file| file.path.ends_with("libfoo.so.1.2.3"))
                .unwrap_or_else(|| panic!("{fmt:?} lost the link target: {decoded:?}"));
            assert!(!target.is_symlink(), "{fmt:?}");
            assert_eq!(target.buffer, b"ELF", "{fmt:?}");
        }
    }

    /// Formats that cannot represent a link must say so rather than silently
    /// storing an empty file in its place.
    #[test]
    fn formats_without_link_support_refuse_links() {
        let link = || File::symlink("a.so".to_string(), "a.so.1".to_string());

        #[cfg(feature = "7z")]
        assert!(
            Fmt::SevenZip.encode(vec![link()]).is_err(),
            "7z must reject symlinks"
        );

        #[cfg(feature = "gz")]
        assert!(
            Fmt::Gz.encode(vec![link()]).is_err(),
            "gzip must reject symlinks"
        );
    }
}
