/// TAR.ZSTD decoding implementation
use crate::{
    File,
    archive::tar::decode::decode_tar_archive,
    error::{ArchiveError, Result},
    traits::Decode,
};
use ruzstd::decoding::StreamingDecoder;
use std::io::{Cursor, Read};

use super::TarZstd;

impl Decode for TarZstd {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let cur = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cur).map_err(|e| {
            ArchiveError::DecompressionError(format!("Failed to create Zstd decoder: {}", e))
        })?;

        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            ArchiveError::DecompressionError(format!("Zstd decompression failed: {}", e))
        })?;

        decode_tar_archive(Cursor::new(decompressed)).map_err(|e| ArchiveError::DecodeFailed {
            format: "tar.zst".to_string(),
            reason: e.to_string(),
        })
    }
}
