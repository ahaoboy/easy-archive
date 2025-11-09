/// TAR archive format implementations
///
/// This module provides support for TAR archives and various compressed TAR formats:
/// - Plain TAR (.tar)
/// - Gzip-compressed TAR (.tar.gz, .tgz)
/// - XZ-compressed TAR (.tar.xz, .txz)
/// - Bzip2-compressed TAR (.tar.bz2, .tbz2)
/// - Zstd-compressed TAR (.tar.zst, .tzst, .tzstd)
use crate::{
    File,
    error::{ArchiveError, Result},
};

#[cfg(feature = "decode")]
use crate::utils::clean;

#[cfg(feature = "decode")]
use crate::traits::Decode;

#[cfg(feature = "encode")]
use crate::traits::Encode;

#[cfg(feature = "encode")]
use crate::{Fmt, utils::check_duplicate_files};

use std::io::Cursor;

#[cfg(feature = "decode")]
use std::io::{BufReader, Read};

#[cfg(feature = "decode")]
use tar::Archive;

/// Common helper function for decoding TAR archives
///
/// This function handles the core TAR decoding logic that is shared across
/// all TAR format variants (plain, gzip, xz, bzip2, zstd).
///
/// # Performance Notes
/// - Uses streaming processing to minimize memory usage
/// - Skips PAX headers to avoid unnecessary processing
/// - Pre-allocates file buffer for each entry
///
/// # Arguments
/// * `reader` - A reader providing the TAR data
///
/// # Returns
/// * `Ok(Vec<File>)` - The extracted files
/// * `Err(ArchiveError)` - If decoding fails
#[cfg(feature = "decode")]
fn decode_tar_archive<R: Read>(reader: R) -> Result<Vec<File>> {
    // Pre-allocate with estimated capacity (typical archives have 10-100 files)
    let mut files = Vec::with_capacity(32);
    let mut archive = Archive::new(reader);

    let entries = archive.entries().map_err(|e| ArchiveError::DecodeFailed {
        format: "tar".to_string(),
        reason: format!("Failed to read tar entries: {}", e),
    })?;

    for entry in entries {
        let mut file = entry.map_err(|e| ArchiveError::DecodeFailed {
            format: "tar".to_string(),
            reason: format!("Failed to read tar entry: {}", e),
        })?;

        let path = file
            .header()
            .path()
            .map_err(|e| ArchiveError::InvalidArchive(format!("Invalid path in tar: {}", e)))?
            .to_string_lossy()
            .to_string();

        // Skip PAX global headers
        if path == "pax_global_header" {
            continue;
        }

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| ArchiveError::DecodeFailed {
                format: "tar".to_string(),
                reason: format!("Failed to read file content: {}", e),
            })?;

        let mode = file.header().mode().ok();
        let is_dir = path.ends_with("/");
        let path = clean(&path);
        let mtime = file.header().mtime().ok();

        files.push(File::new(path, buffer, mode, is_dir, mtime));
    }

    Ok(files)
}

/// Common helper function for encoding TAR archives
///
/// This function handles the core TAR decoding logic that is shared across
/// all TAR format variants. It includes duplicate file detection.
///
/// # Performance Notes
/// - Checks for duplicates before encoding to fail fast
/// - Uses buffered writing for better I/O performance
/// - Pre-allocates buffer based on estimated archive size
///
/// # Arguments
/// * `files` - The files to include in the archive
///
/// # Returns
/// * `Ok(Vec<u8>)` - The encoded TAR data
/// * `Err(ArchiveError)` - If encoding fails or duplicates are detected
#[cfg(feature = "encode")]
fn encode_tar_archive(files: Vec<File>) -> Result<Vec<u8>> {
    // Check for duplicate files before encoding (fail fast)
    check_duplicate_files(&files)?;

    // Pre-allocate buffer with estimated size (sum of file sizes + 512 bytes per file for headers)
    let estimated_size: usize = files.iter().map(|f| f.buffer.len() + 512).sum();
    let mut buffer: Vec<u8> = Vec::with_capacity(estimated_size);
    {
        let mut builder = tar::Builder::new(&mut buffer);

        for file in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(file.buffer.len() as u64);
            header.set_mode(file.mode.unwrap_or(0o644));
            header.set_uid(0);
            header.set_gid(0);
            header.set_mtime(file.last_modified.unwrap_or(0));
            header.set_cksum();

            builder
                .append_data(&mut header, &file.path, &file.buffer[..])
                .map_err(|e| ArchiveError::EncodeFailed {
                    format: "tar".to_string(),
                    reason: format!("Failed to append file '{}': {}", file.path, e),
                })?;
        }

        builder.finish().map_err(|e| ArchiveError::EncodeFailed {
            format: "tar".to_string(),
            reason: format!("Failed to finalize tar archive: {}", e),
        })?;
    }

    Ok(buffer)
}

// ============================================================================
// Plain TAR format
// ============================================================================

#[cfg(feature = "tar")]
/// Plain TAR archive format handler
pub struct Tar;

#[cfg(all(feature = "tar", feature = "decode"))]
impl Decode for Tar {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let cur = Cursor::new(buffer);
        decode_tar_archive(cur)
    }
}

#[cfg(all(feature = "tar", feature = "encode"))]
impl Encode for Tar {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        encode_tar_archive(files)
    }
}

// ============================================================================
// Gzip-compressed TAR format
// ============================================================================

#[cfg(all(feature = "tar-gz", feature = "decode"))]
use flate2::read::GzDecoder;
#[cfg(feature = "encode")]
use flate2::{Compression, bufread::GzEncoder};

#[cfg(feature = "tar-gz")]
/// Gzip-compressed TAR archive format handler
pub struct TarGz;

#[cfg(all(feature = "tar-gz", feature = "decode"))]
impl Decode for TarGz {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let buffer = buffer.as_ref();
        let decoder = GzDecoder::new(buffer);

        // Pre-allocate decompression buffer (gzip typically achieves 2-3x compression)
        let estimated_size = buffer.len() * 3;
        let mut decompressed = Vec::with_capacity(estimated_size);

        // Use BufReader for better I/O performance
        let mut buf_reader = BufReader::new(decoder);
        buf_reader.read_to_end(&mut decompressed).map_err(|e| {
            ArchiveError::DecompressionError(format!("GZ decompression failed: {}", e))
        })?;

        Tar::decode(decompressed).map_err(|e| ArchiveError::DecodeFailed {
            format: "tar.gz".to_string(),
            reason: e.to_string(),
        })
    }
}

#[cfg(all(feature = "tar-gz", feature = "encode"))]
impl Encode for TarGz {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        use std::io::Read;
        let tar = Fmt::Tar.encode(files)?;

        // Pre-allocate compressed buffer (estimate 30-40% of original size)
        let estimated_size = tar.len() / 3;
        let mut compressed = Vec::with_capacity(estimated_size);

        let mut cursor = Cursor::new(tar);

        // Use default compression level (6) for balanced speed/compression
        let mut encoder = GzEncoder::new(&mut cursor, Compression::default());

        encoder
            .read_to_end(&mut compressed)
            .map_err(|e| ArchiveError::CompressionError(format!("GZ compression failed: {}", e)))?;

        Ok(compressed)
    }
}

// ============================================================================
// XZ-compressed TAR format
// ============================================================================

#[cfg(all(feature = "tar-xz", feature = "decode"))]
use liblzma::bufread::XzDecoder;
#[cfg(all(feature = "tar-xz", feature = "encode"))]
use liblzma::write::XzEncoder;

#[cfg(feature = "tar-xz")]
/// XZ-compressed TAR archive format handler
pub struct TarXz;

#[cfg(feature = "decode")]
#[cfg(all(feature = "tar-xz", feature = "decode"))]
impl Decode for TarXz {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let buffer = buffer.as_ref();
        let mut decoder = XzDecoder::new(buffer);
        let mut decompressed = Vec::new();

        decoder.read_to_end(&mut decompressed).map_err(|e| {
            ArchiveError::DecompressionError(format!("XZ decompression failed: {}", e))
        })?;

        Tar::decode(decompressed).map_err(|e| ArchiveError::DecodeFailed {
            format: "tar.xz".to_string(),
            reason: e.to_string(),
        })
    }
}

#[cfg(all(feature = "tar-xz", feature = "encode"))]
impl Encode for TarXz {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        let tar = Fmt::Tar.encode(files)?;

        // Encode with compression level 6 (balanced speed/compression)
        use std::io::{Cursor, Write};
        let cursor = Cursor::new(Vec::new());
        let mut encoder = XzEncoder::new(cursor, 6);

        encoder
            .write_all(&tar)
            .map_err(|e| ArchiveError::CompressionError(format!("XZ compression failed: {}", e)))?;

        let cursor = encoder.finish().map_err(|e| {
            ArchiveError::CompressionError(format!("XZ finalization failed: {}", e))
        })?;

        Ok(cursor.into_inner())
    }
}

// ============================================================================
// Bzip2-compressed TAR format
// ============================================================================

#[cfg(all(feature = "tar-bz", feature = "decode"))]
use bzip2_rs::DecoderReader;

#[cfg(feature = "tar-bz")]
/// Bzip2-compressed TAR archive format handler
pub struct TarBz;

#[cfg(feature = "decode")]
#[cfg(all(feature = "tar-bz", feature = "decode"))]
impl Decode for TarBz {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let cur = Cursor::new(buffer);
        let reader = BufReader::new(DecoderReader::new(cur));
        let decompressed: Result<Vec<u8>> = reader
            .bytes()
            .collect::<std::io::Result<Vec<u8>>>()
            .map_err(|e| {
                ArchiveError::DecompressionError(format!("BZ2 decompression failed: {}", e))
            });

        let decompressed = decompressed?;

        Tar::decode(decompressed).map_err(|e| ArchiveError::DecodeFailed {
            format: "tar.bz2".to_string(),
            reason: e.to_string(),
        })
    }
}
#[cfg(all(feature = "tar-bz", feature = "encode"))]
impl Encode for TarBz {
    fn encode(_files: Vec<File>) -> Result<Vec<u8>> {
        // The bzip2-rs library does not provide an encoder API
        Err(ArchiveError::UnsupportedFormat(
            "TAR.BZ2 encoding is not supported (bzip2-rs lacks encoder)".to_string(),
        ))
    }
}

// ============================================================================
// Zstd-compressed TAR format
// ============================================================================

#[cfg(all(feature = "tar-zstd", feature = "decode"))]
use ruzstd::decoding::StreamingDecoder;

#[cfg(feature = "tar-zstd")]
/// Zstd-compressed TAR archive format handler
pub struct TarZstd;
#[cfg(feature = "decode")]
#[cfg(all(feature = "tar-zstd", feature = "decode"))]
impl Decode for TarZstd {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let cur = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cur).map_err(|e| {
            ArchiveError::DecompressionError(format!("Failed to create Zstd decoder: {}", e))
        })?;

        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            ArchiveError::DecompressionError(format!("Zstd decompression failed: {}", e))
        })?;

        Tar::decode(decompressed).map_err(|e| ArchiveError::DecodeFailed {
            format: "tar.zst".to_string(),
            reason: e.to_string(),
        })
    }
}

#[cfg(all(feature = "tar-zstd", feature = "encode"))]
impl Encode for TarZstd {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        let tar = Fmt::Tar.encode(files)?;
        let mut cursor = Cursor::new(tar);
        let mut compressed = Vec::new();

        {
            // Use compression level 6 for balanced speed/compression
            let mut encoder = zstd::Encoder::new(&mut compressed, 6).map_err(|e| {
                ArchiveError::CompressionError(format!("Failed to create Zstd encoder: {}", e))
            })?;

            std::io::copy(&mut cursor, &mut encoder).map_err(|e| {
                ArchiveError::CompressionError(format!("Zstd compression failed: {}", e))
            })?;

            encoder.finish().map_err(|e| {
                ArchiveError::CompressionError(format!("Zstd finalization failed: {}", e))
            })?;
        }

        Ok(compressed)
    }
}
