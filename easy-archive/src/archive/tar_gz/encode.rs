/// TAR.GZ encoding implementation
use crate::{
    File, Fmt,
    error::{ArchiveError, Result},
    traits::Encode,
};
use flate2::{Compression, bufread::GzEncoder};
use std::io::{Cursor, Read};

use super::TarGz;

impl Encode for TarGz {
    fn encode(files: Vec<File>) -> Result<Vec<u8>> {
        let tar = Fmt::Tar.encode(files)?;

        // Pre-allocate compressed buffer (estimate 30-40% of original size)
        let estimated_size = tar.len() / 3;
        let mut cursor = Cursor::new(tar);

        // Use default compression level (6) for balanced speed/compression
        let mut encoder = GzEncoder::new(&mut cursor, Compression::default());
        let mut compressed = Vec::with_capacity(estimated_size);

        encoder
            .read_to_end(&mut compressed)
            .map_err(|e| ArchiveError::CompressionError(format!("GZ compression failed: {}", e)))?;

        Ok(compressed)
    }
}
