/// Single-file gzip (.gz) decoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Decode,
};
use flate2::read::GzDecoder;
use std::io::{BufReader, Read};

use super::Gz;

/// Parsed metadata from a gzip header.
struct GzHeader {
    /// Original filename stored in the `FNAME` field, if present.
    filename: Option<String>,
    /// Modification time (Unix seconds) from the `MTIME` field, if non-zero.
    mtime: Option<u64>,
}

/// Parse the gzip header to extract the optional original filename (`FNAME`)
/// and modification time (`MTIME`).
///
/// Returns `None` if the buffer is too short to contain a valid header.
/// The compressed payload itself is still decoded by [`GzDecoder`]; this only
/// reads the up-front metadata fields.
fn parse_header(bytes: &[u8]) -> Option<GzHeader> {
    // A minimal gzip header is 10 bytes: magic(2) CM(1) FLG(1) MTIME(4) XFL(1) OS(1)
    if bytes.len() < 10 || bytes[0] != 0x1f || bytes[1] != 0x8b {
        return None;
    }
    let flg = bytes[3];
    let mtime_raw = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let mtime = if mtime_raw == 0 {
        None
    } else {
        Some(mtime_raw as u64)
    };

    let mut idx = 10;

    // FEXTRA (bit 2): 2-byte little-endian length followed by that many bytes.
    if flg & 0x04 != 0 {
        if idx + 2 > bytes.len() {
            return None;
        }
        let xlen = u16::from_le_bytes([bytes[idx], bytes[idx + 1]]) as usize;
        idx += 2 + xlen;
    }

    // FNAME (bit 3): original filename, NUL-terminated.
    let mut filename = None;
    if flg & 0x08 != 0 && idx < bytes.len() {
        let start = idx;
        while idx < bytes.len() && bytes[idx] != 0 {
            idx += 1;
        }
        let name = String::from_utf8_lossy(&bytes[start..idx]).to_string();
        if !name.is_empty() {
            filename = Some(name);
        }
    }

    Some(GzHeader { filename, mtime })
}

impl Decode for Gz {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Result<Vec<File>> {
        let buffer = buffer.as_ref();
        let header = parse_header(buffer);

        let decoder = GzDecoder::new(buffer);
        let mut buf_reader = BufReader::new(decoder);
        let mut decompressed = Vec::new();
        buf_reader.read_to_end(&mut decompressed).map_err(|e| {
            ArchiveError::DecompressionError(format!("GZ decompression failed: {}", e))
        })?;

        // The path falls back to the gzip header's original filename when
        // present; otherwise the caller is responsible for supplying a name
        // (a bare gzip stream carries no inherent path).
        let path = header
            .as_ref()
            .and_then(|h| h.filename.clone())
            .unwrap_or_default();

        Ok(vec![File {
            path,
            buffer: decompressed,
            mode: None,
            is_dir: false,
            last_modified: header.as_ref().and_then(|h| h.mtime),
        }])
    }
}
