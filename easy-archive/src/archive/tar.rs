use crate::{Decode, Encode, File, tool::clean};
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

impl Encode for Tar {}
impl Encode for TarGz {}
impl Encode for TarXz {}
impl Encode for TarBz {}
impl Encode for TarZstd {}
