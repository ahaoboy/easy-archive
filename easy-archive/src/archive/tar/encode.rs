/// TAR encoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Encode,
    utils::check_duplicate_files,
};

use super::Tar;

/// Common helper function for encoding TAR archives
///
/// This function handles the core TAR encoding logic that is shared across
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
pub(crate) fn encode_tar_archive(files: Vec<File>) -> Result<Vec<u8>> {
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

impl Encode for Tar {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        encode_tar_archive(files)
    }
}
