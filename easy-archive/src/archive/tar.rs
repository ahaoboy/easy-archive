use crate::{
    tool::clean,
    ty::{Decode, File, Files},
};
use std::io::{Cursor, Read};
use tar::Archive;

pub struct Tar;

impl Decode for Tar {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        let mut files = Files::new();
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
            files.insert(path.clone(), File::new(path, buffer, mode, is_dir));
        }
        Some(files)
    }
}
use flate2::read::GzDecoder;

pub struct TarGz;
impl Decode for TarGz {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        let mut decoder = GzDecoder::new(&buffer[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).ok()?;
        Tar::decode(decompressed)
    }
}

#[cfg(any(test, feature = "xz2"))]
fn decode_xz2(buffer: &[u8]) -> Option<Files> {
    use xz2::bufread::XzDecoder;
    let mut dec = XzDecoder::new(buffer);
    let mut decompressed = vec![];
    dec.read_to_end(&mut decompressed).ok()?;
    Tar::decode(decompressed)
}

fn decode_lzma(buffer: &[u8]) -> Option<Files> {
    let mut cur = Cursor::new(buffer);
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::xz_decompress(&mut cur, &mut decomp).ok()?;
    Tar::decode(decomp)
}

pub struct TarXz;
impl Decode for TarXz {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        let files = decode_lzma(&buffer);
        #[cfg(feature = "xz2")]
        if files.is_none() {
            return decode_xz2(&buffer);
        }
        files
    }
}

pub struct TarBz;
impl Decode for TarBz {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        use bzip2_rs::DecoderReader;
        let cur = Cursor::new(buffer);
        let reader = DecoderReader::new(cur);
        let v = reader.bytes().map(|i| i.unwrap()).collect();
        Tar::decode(v)
    }
}
use ruzstd::decoding::StreamingDecoder;

pub struct TarZstd;
impl Decode for TarZstd {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        let cur = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cur).unwrap();
        let mut result = Vec::new();
        decoder.read_to_end(&mut result).unwrap();
        Tar::decode(result)
    }
}
