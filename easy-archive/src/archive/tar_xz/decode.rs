/// TAR.XZ decoding implementation
use crate::{
    File,
    archive::tar::decode::decode_tar_archive,
    error::{ArchiveError, Result},
    traits::Decode,
};
use liblzma::bufread::XzDecoder;
use std::io::Read;

use super::TarXz;

impl Decode for TarXz {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let buffer = buffer.as_ref();
        let mut decoder = XzDecoder::new(buffer);
        let mut decompressed = Vec::new();

        decoder.read_to_end(&mut decompressed).map_err(|e| {
            ArchiveError::DecompressionError(format!("XZ decompression failed: {}", e))
        })?;

        decode_tar_archive(std::io::Cursor::new(decompressed)).map_err(|e| {
            ArchiveError::DecodeFailed {
                format: "tar.xz".to_string(),
                reason: e.to_string(),
            }
        })
    }
}
