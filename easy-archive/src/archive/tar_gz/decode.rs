/// TAR.GZ decoding implementation
use crate::{
    File,
    archive::tar::decode::decode_tar_archive,
    error::{ArchiveError, Result},
    traits::Decode,
};
use flate2::read::GzDecoder;
use std::io::{BufReader, Read};

use super::TarGz;

impl Decode for TarGz {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let buffer = buffer.as_ref();
        let decoder = GzDecoder::new(buffer);

        // Pre-allocate decompression buffer (gzip typically achieves 2-3x compression)
        let estimated_size = buffer.len() * 3;
        let mut decompressed = Vec::with_capacity(estimated_size);

        // Use BufReader for better I/O performance
        let mut buf_reader = BufReader::new(decoder);
        buf_reader.read_to_end(&mut decompressed).map_err(|e| {
            ArchiveError::DecompressionError(format!("GZ decompression failed: {}", e))
        })?;

        decode_tar_archive(std::io::Cursor::new(decompressed)).map_err(|e| {
            ArchiveError::DecodeFailed {
                format: "tar.gz".to_string(),
                reason: e.to_string(),
            }
        })
    }
}
