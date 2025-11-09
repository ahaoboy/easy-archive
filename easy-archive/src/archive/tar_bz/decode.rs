/// TAR.BZ2 decoding implementation
use crate::{
    File,
    archive::tar::decode::decode_tar_archive,
    error::{ArchiveError, Result},
    traits::Decode,
};
use bzip2_rs::DecoderReader;
use std::io::{BufReader, Cursor, Read};

use super::TarBz;

impl Decode for TarBz {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let cur = Cursor::new(buffer);
        let reader = BufReader::new(DecoderReader::new(cur));
        let decompressed: Result<Vec<u8>> = reader
            .bytes()
            .collect::<std::io::Result<Vec<u8>>>()
            .map_err(|e| {
                ArchiveError::DecompressionError(format!("BZ2 decompression failed: {}", e))
            });

        let decompressed = decompressed?;

        decode_tar_archive(Cursor::new(decompressed)).map_err(|e| ArchiveError::DecodeFailed {
            format: "tar.bz2".to_string(),
            reason: e.to_string(),
        })
    }
}
