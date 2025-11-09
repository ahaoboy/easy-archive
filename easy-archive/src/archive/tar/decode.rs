/// TAR decoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Decode,
    utils::clean,
};
use std::io::{Cursor, Read};
use tar::Archive;

use super::Tar;

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
pub(crate) fn decode_tar_archive<R: Read>(reader: R) -> Result<Vec<File>> {
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

impl Decode for Tar {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let cur = Cursor::new(buffer);
        decode_tar_archive(cur)
    }
}
