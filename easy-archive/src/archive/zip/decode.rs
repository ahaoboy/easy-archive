/// ZIP decoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Decode,
    utils::clean,
};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use super::Zip;

impl Decode for Zip {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let buffer = buffer.as_ref();

        // Pre-allocate cursor buffer to avoid reallocation
        let mut cursor = Cursor::new(Vec::with_capacity(buffer.len()));

        cursor
            .write_all(buffer)
            .map_err(|e| ArchiveError::DecodeFailed {
                format: "zip".to_string(),
                reason: format!("Failed to write buffer: {}", e),
            })?;

        cursor
            .seek(SeekFrom::Start(0))
            .map_err(|e| ArchiveError::DecodeFailed {
                format: "zip".to_string(),
                reason: format!("Failed to seek: {}", e),
            })?;

        // Pre-allocate files vector (typical zip has 10-100 files)
        let mut files = Vec::with_capacity(32);
        let mut archive = zip::ZipArchive::new(cursor).map_err(|e| ArchiveError::DecodeFailed {
            format: "zip".to_string(),
            reason: format!("Failed to open zip archive: {}", e),
        })?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| ArchiveError::DecodeFailed {
                    format: "zip".to_string(),
                    reason: format!("Failed to read entry {}: {}", i, e),
                })?;

            let path = file.name().to_string();
            let is_dir = file.is_dir() || path.ends_with("/");

            // Read file content (empty for directories)
            let mut buffer = Vec::new();
            if file.is_file() {
                file.read_to_end(&mut buffer)
                    .map_err(|e| ArchiveError::DecodeFailed {
                        format: "zip".to_string(),
                        reason: format!("Failed to read file '{}': {}", path, e),
                    })?;
            }

            let path = clean(&path);
            let last_modified = file
                .last_modified()
                .and_then(|dt| time::OffsetDateTime::try_from(dt).ok())
                .map(|dt| dt.unix_timestamp() as u64);

            files.push(File::new(path, buffer, None, is_dir, last_modified));
        }

        Ok(files)
    }
}
