/// Single-file gzip format (.gz)
///
/// Unlike [`crate::archive::tar_gz::TarGz`], this handles a *plain* gzip
/// stream that wraps a single file rather than a tar archive. Decoding
/// produces exactly one [`crate::File`]; encoding accepts exactly one file.
#[cfg(feature = "gz")]
pub struct Gz;

#[cfg(all(feature = "gz", feature = "decode"))]
mod decode;

#[cfg(all(feature = "gz", feature = "encode"))]
mod encode;

#[cfg(all(test, feature = "gz", feature = "encode", feature = "decode"))]
mod tests {
    use crate::{File, Fmt};

    #[test]
    fn gz_roundtrip_preserves_name_and_content() {
        let content = b"hello gzip world".to_vec();
        let file = File {
            path: "dir/tool.exe".to_string(),
            buffer: content.clone(),
            ..Default::default()
        };

        let encoded = Fmt::Gz.encode(vec![file]).expect("encode failed");
        // gzip magic bytes
        assert_eq!(&encoded[..2], &[0x1f, 0x8b]);

        let decoded = Fmt::Gz.decode(encoded).expect("decode failed");
        assert_eq!(decoded.len(), 1);
        // Only the basename is stored in the gzip FNAME field.
        assert_eq!(decoded[0].path, "tool.exe");
        assert_eq!(decoded[0].buffer, content);
    }

    #[test]
    fn gz_decode_without_filename() {
        // gzip data produced elsewhere may omit the FNAME field. The decoded
        // entry then has an empty path, leaving naming to the caller.
        let bytes = {
            use flate2::{Compression, write::GzEncoder};
            use std::io::Write;
            let mut enc = GzEncoder::new(Vec::new(), Compression::default());
            enc.write_all(b"payload").unwrap();
            enc.finish().unwrap()
        };

        let decoded = Fmt::Gz.decode(bytes).expect("decode failed");
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].path.is_empty());
        assert_eq!(decoded[0].buffer, b"payload");
    }

    #[test]
    fn gz_encode_rejects_multiple_files() {
        let files = vec![
            File {
                path: "a".to_string(),
                buffer: b"a".to_vec(),
                ..Default::default()
            },
            File {
                path: "b".to_string(),
                buffer: b"b".to_vec(),
                ..Default::default()
            },
        ];
        assert!(Fmt::Gz.encode(files).is_err());
    }
}
