use crate::{Decode, Encode, File, Fmt, tool::clean};
use flate2::read::GzDecoder;
use std::io::{BufReader, Cursor, Read};
use tar::Archive;

pub struct Tar;

impl Decode for Tar {
    fn decode(buffer: Vec<u8>) -> Option<Vec<File>> {
        let mut files = Vec::new();
        let cur = Cursor::new(buffer);
        let mut a = Archive::new(cur);
        for file in a.entries().unwrap() {
            let mut file = file.unwrap();
            let path = file.header().path().unwrap().to_string_lossy().to_string();
            // FIXME: skip PAX
            if path == "pax_global_header" {
                continue;
            }
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).expect("failed to read file");
            let mode = file.header().mode().ok();
            let is_dir = path.ends_with("/");
            let path = clean(&path);
            let mtime = file.header().mtime().ok();
            files.push(File::new(path, buffer, mode, is_dir, mtime));
        }
        Some(files)
    }
}

pub struct TarGz;
impl Decode for TarGz {
    fn decode(buffer: Vec<u8>) -> Option<Vec<File>> {
        let mut decoder = GzDecoder::new(&buffer[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).ok()?;
        Tar::decode(decompressed)
    }
}

#[cfg(feature = "liblzma")]
fn decode_xz2(buffer: &[u8]) -> Option<Vec<File>> {
    use liblzma::bufread::XzDecoder;
    let mut dec = XzDecoder::new(buffer);
    let mut decompressed = vec![];
    dec.read_to_end(&mut decompressed).ok()?;
    Tar::decode(decompressed)
}

#[cfg(feature = "liblzma")]
/// This is intended to be used by most for encoding data. The `preset`
/// argument is a number 0-9 indicating the compression level to use, and
/// normally 6 is a reasonable default.
fn compress_xz(data: &[u8], preset: u32) -> std::io::Result<Vec<u8>> {
    use liblzma::write::XzEncoder;
    use std::io::{Cursor, Write};
    let cursor = Cursor::new(Vec::new());
    let mut encoder = XzEncoder::new(cursor, preset);
    encoder.write_all(data)?;
    let cursor = encoder.finish()?;
    Ok(cursor.into_inner())
}

#[cfg(feature = "lzma-rs")]
fn decode_lzma_rs(buffer: &[u8]) -> Option<Vec<File>> {
    let mut cur = Cursor::new(buffer);
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::xz_decompress(&mut cur, &mut decomp).ok()?;
    Tar::decode(decomp)
}

pub struct TarXz;
impl Decode for TarXz {
    fn decode(buffer: Vec<u8>) -> Option<Vec<File>> {
        #[cfg(feature = "liblzma")]
        return decode_xz2(&buffer);
        #[allow(unreachable_code)]
        #[cfg(feature = "lzma-rs")]
        return decode_lzma_rs(&buffer);
    }
}

pub struct TarBz;
impl Decode for TarBz {
    fn decode(buffer: Vec<u8>) -> Option<Vec<File>> {
        use bzip2_rs::DecoderReader;
        let cur = Cursor::new(buffer);
        let reader = BufReader::new(DecoderReader::new(cur));
        let v = reader.bytes().map(|i| i.unwrap()).collect();
        Tar::decode(v)
    }
}
use ruzstd::decoding::StreamingDecoder;

pub struct TarZstd;
impl Decode for TarZstd {
    fn decode(buffer: Vec<u8>) -> Option<Vec<File>> {
        let cur = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cur).unwrap();
        let mut result = Vec::new();
        decoder.read_to_end(&mut result).unwrap();
        Tar::decode(result)
    }
}

impl Encode for Tar {
    fn encode(files: Vec<File>) -> Option<Vec<u8>> {
        use tar::Header;
        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut buffer);

            for file in files {
                let mut header = Header::new_gnu();
                header.set_size(file.buffer.len() as u64);
                header.set_mode(file.mode.unwrap_or(0o644));
                header.set_uid(0);
                header.set_gid(0);
                header.set_mtime(file.last_modified.unwrap_or(0));
                header.set_cksum();
                builder
                    .append_data(&mut header, &file.path, &file.buffer[..])
                    .ok()?;
            }

            builder.finish().ok()?;
        }
        Some(buffer)
    }
}

impl Encode for TarGz {}
impl Encode for TarXz {
    #[cfg(feature = "liblzma")]
    fn encode(_files: Vec<File>) -> Option<Vec<u8>> {
        let tar = Fmt::Tar.encode(_files)?;
        let xz = compress_xz(&tar, 6);
        xz.ok()
    }
}
impl Encode for TarBz {}
impl Encode for TarZstd {}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{File, Fmt};

    #[test]
    fn xz_encode() {
        let mut v = vec![];
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let asset_dir = base.join("../assets");
        for i in std::fs::read_dir(asset_dir).expect("read dir error") {
            let file_path = i.expect("get path error").path();
            let path = file_path
                .file_name()
                .expect("get name error")
                .to_string_lossy()
                .to_string();
            let buffer = std::fs::read(&file_path).expect("read file error");

            v.push(File {
                buffer,
                path,
                mode: None,
                is_dir: false,
                last_modified: None,
            })
        }

        let xz = Fmt::TarXz.encode(v).expect("zip error");

        println!("xz {}", xz.len());
        assert!(xz.len() > 0);
    }
}
