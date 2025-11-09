/// TAR.ZSTD encoding implementation
use crate::{
    File, Fmt,
    error::{ArchiveError, Result},
    traits::Encode,
};
use std::io::Cursor;

use super::TarZstd;

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
