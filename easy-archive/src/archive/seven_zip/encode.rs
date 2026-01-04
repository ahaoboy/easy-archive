/// ZIP encoding implementation
use crate::{ArchiveError, File, error::Result, traits::Encode};
use sevenz_rust2::{ArchiveEntry, ArchiveWriter, NtTime, SourceReader};
use std::io::Cursor;

use super::SevenZip;

impl Encode for SevenZip {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        let mut output = vec![];
        let cursor = Cursor::new(&mut output);

        let mut w = ArchiveWriter::new(cursor).map_err(|e| ArchiveError::EncodeFailed {
            format: "7z".to_string(),
            reason: format!("Failed to create ArchiveWriter: {}", e),
        })?;

        let mut entries = vec![];
        let mut readers = vec![];

        for file in files {
            entries.push(ArchiveEntry {
                name: file.path.replace("\\", "/"),
                has_stream: true,
                is_directory: file.is_dir,
                has_last_modified_date: file.last_modified.is_some(),
                last_modified_date: NtTime::new(file.last_modified.unwrap_or_default()),
                ..Default::default()
            });
            readers.push(SourceReader::new(Cursor::new(file.buffer)));
        }
        w.push_archive_entries(entries, readers)
            .map_err(|e| ArchiveError::EncodeFailed {
                format: "7z".to_string(),
                reason: format!("Failed to push_archive_entries: {}", e),
            })?;
        w.finish().map_err(|e| ArchiveError::EncodeFailed {
            format: "7z".to_string(),
            reason: format!("Failed to finish: {}", e),
        })?;
        Ok(output)
    }
}
