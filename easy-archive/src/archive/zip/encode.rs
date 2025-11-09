/// ZIP encoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Encode,
    utils::check_duplicate_files,
};
use std::collections::HashSet;
use std::io::{Cursor, Write};
use time::OffsetDateTime;
use zip::DateTime;

use super::Zip;

impl Encode for Zip {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        // Check for duplicate files before encoding (fail fast)
        check_duplicate_files(&files)?;

        // Pre-allocate output buffer with estimated size
        // ZIP typically achieves 40-60% compression with Zstd
        let estimated_size: usize = files.iter().map(|f| f.buffer.len()).sum::<usize>() / 2;
        let mut output = Vec::with_capacity(estimated_size);
        let cursor = Cursor::new(&mut output);
        let mut zip = zip::ZipWriter::new(cursor);
        let mut dir_set = HashSet::with_capacity(files.len() / 4); // Estimate directory count

        // Helper function to create file options with timestamp
        // Performance: Zstd provides excellent compression speed and ratio
        let create_options = |last_modified: Option<u64>| -> zip::write::FullFileOptions {
            let mut options = zip::write::FullFileOptions::default()
                // Use Zstd for better compression/speed balance (faster than LZMA, better than Deflate)
                .compression_method(zip::CompressionMethod::Zstd);

            if let Some(timestamp) = last_modified
                && let Ok(offset_time) = OffsetDateTime::from_unix_timestamp(timestamp as i64)
                && let Ok(datetime) = DateTime::try_from(offset_time)
            {
                options = options.last_modified_time(datetime);
            }

            options
        };

        // First pass: Create all explicit directories
        for file in files.iter().filter(|f| f.is_dir) {
            if dir_set.contains(&file.path) {
                continue;
            }

            dir_set.insert(file.path.clone());
            let options = create_options(file.last_modified);

            zip.add_directory(&file.path, options)
                .map_err(|e| ArchiveError::EncodeFailed {
                    format: "zip".to_string(),
                    reason: format!("Failed to add directory '{}': {}", file.path, e),
                })?;
        }

        // Second pass: Create implicit parent directories for files
        for file in files.iter().filter(|f| !f.is_dir) {
            if !file.path.contains('/') {
                continue;
            }

            if let Some(parent) = std::path::Path::new(&file.path).parent() {
                let parent_path = parent.to_string_lossy().to_string();
                if !parent_path.is_empty() && !dir_set.contains(&parent_path) {
                    dir_set.insert(parent_path.clone());
                    let options = create_options(file.last_modified);

                    zip.add_directory(&parent_path, options).map_err(|e| {
                        ArchiveError::EncodeFailed {
                            format: "zip".to_string(),
                            reason: format!(
                                "Failed to add parent directory '{}': {}",
                                parent_path, e
                            ),
                        }
                    })?;
                }
            }
        }

        // Third pass: Add all files
        for file in files.iter().filter(|f| !f.is_dir) {
            let mode = file.mode.unwrap_or(0o755);
            let options = create_options(file.last_modified).unix_permissions(mode);

            zip.start_file(&file.path, options)
                .map_err(|e| ArchiveError::EncodeFailed {
                    format: "zip".to_string(),
                    reason: format!("Failed to start file '{}': {}", file.path, e),
                })?;

            zip.write_all(&file.buffer)
                .map_err(|e| ArchiveError::EncodeFailed {
                    format: "zip".to_string(),
                    reason: format!("Failed to write file '{}': {}", file.path, e),
                })?;
        }

        zip.finish().map_err(|e| ArchiveError::EncodeFailed {
            format: "zip".to_string(),
            reason: format!("Failed to finalize zip archive: {}", e),
        })?;

        Ok(output)
    }
}
