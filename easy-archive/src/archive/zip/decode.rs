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

            let path = file.name()?.to_string();
            let is_symlink = file.is_symlink();
            let is_dir = file.is_dir() || path.ends_with("/");

            // Read file content (empty for directories). A symlink stores its
            // target as content, so it has to be read too.
            let mut buffer = Vec::new();
            if file.is_file() || is_symlink {
                file.read_to_end(&mut buffer)
                    .map_err(|e| ArchiveError::DecodeFailed {
                        format: "zip".to_string(),
                        reason: format!("Failed to read file '{}': {}", path, e),
                    })?;
            }

            let link_target = if is_symlink {
                Some(String::from_utf8_lossy(&buffer).to_string())
            } else {
                None
            };
            let buffer = if is_symlink { Vec::new() } else { buffer };

            let path = clean(&path);
            let last_modified = file
                .last_modified()
                .and_then(|dt| time::PrimitiveDateTime::try_from(dt).ok())
                .map(|pt| pt.assume_utc().unix_timestamp() as u64);

            files.push(File {
                path,
                buffer,
                mode: None,
                is_dir,
                link_target,
                last_modified,
            });
        }

        Ok(files)
    }
}
