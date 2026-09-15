/// Single-file gzip (.gz) encoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Encode,
};
use flate2::{Compression, GzBuilder};
use std::io::Write;

use super::Gz;

impl Encode for Gz {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        if files.len() != 1 {
            return Err(ArchiveError::EncodeFailed {
                format: "gz".to_string(),
                reason: format!(
                    "gzip wraps a single file, but {} were provided",
                    files.len()
                ),
            });
        }

        let file = files.into_iter().next().unwrap();

        // Record the original filename (basename) so that decoding can
        // recover it. A bare gzip stream has no other place to store a path.
        let filename = file.path.rsplit(['/', '\\']).next().unwrap_or(&file.path);

        let mut encoder = GzBuilder::new()
            .filename(filename)
            .write(Vec::new(), Compression::default());

        encoder.write_all(&file.buffer).map_err(|e| {
            ArchiveError::CompressionError(format!("GZ compression failed: {}", e))
        })?;

        encoder
            .finish()
            .map_err(|e| ArchiveError::CompressionError(format!("GZ compression failed: {}", e)))
    }
}
