use crate::ty::{Decode, File, Files};
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
            files.insert(path.clone(), File::new(path, buffer, None));
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

pub struct TarXz;
impl Decode for TarXz {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        let mut cur = Cursor::new(buffer);
        let mut decomp: Vec<u8> = Vec::new();
        lzma_rs::xz_decompress(&mut cur, &mut decomp).unwrap();
        Tar::decode(decomp)
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
