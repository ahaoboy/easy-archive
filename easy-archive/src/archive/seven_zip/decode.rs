use sevenz_rust2::{ArchiveReader, Password};

/// ZIP decoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Decode,
};
use std::io::{Cursor, Seek, SeekFrom, Write};

use super::SevenZip;

impl Decode for SevenZip {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let buffer = buffer.as_ref();

        // Pre-allocate cursor buffer to avoid reallocation
        let mut cursor = Cursor::new(Vec::with_capacity(buffer.len()));

        cursor
            .write_all(buffer)
            .map_err(|e| ArchiveError::DecodeFailed {
                format: "7z".to_string(),
                reason: format!("Failed to write buffer: {}", e),
            })?;

        cursor
            .seek(SeekFrom::Start(0))
            .map_err(|e| ArchiveError::DecodeFailed {
                format: "7z".to_string(),
                reason: format!("Failed to seek: {}", e),
            })?;

        // Pre-allocate files vector (typical zip has 10-100 files)
        let mut files = Vec::with_capacity(32);

        let mut seven = ArchiveReader::new(cursor, Password::empty()).unwrap();
        seven
            .for_each_entries(|entry, reader| {
                let mut buffer = Vec::with_capacity(entry.size as usize);
                reader.read_to_end(&mut buffer)?;
                let file = File {
                    buffer,
                    path: entry.name.to_string(),
                    mode: None,
                    is_dir: entry.is_directory,
                    last_modified: None,
                };
                files.push(file);
                Ok(true)
            })
            .unwrap();

        Ok(files)
    }
}
