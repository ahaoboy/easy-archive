/// TAR.XZ encoding implementation
use crate::{
    File, Fmt,
    error::{ArchiveError, Result},
    traits::Encode,
};
use liblzma::write::XzEncoder;
use std::io::{Cursor, Write};

use super::TarXz;

impl Encode for TarXz {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        let tar = Fmt::Tar.encode(files)?;

        // Encode with compression level 6 (balanced speed/compression)
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
