use crate::ty::{Decode, File, Files};
use std::io::{Cursor, Read};
use tar::Archive;

pub struct Tar;

impl Decode for Tar {
    fn decode(buf: Vec<u8>) -> Option<Files> {
        let mut files = Files::new();
        let cur = Cursor::new(buf);
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
    fn decode(buf: Vec<u8>) -> Option<Files> {
        let mut decoder = GzDecoder::new(&buf[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).ok()?;
        Tar::decode(decompressed)
    }
}

pub struct TarXz;
impl Decode for TarXz {
    fn decode(buf: Vec<u8>) -> Option<Files> {
        let mut cur = Cursor::new(buf);
        let mut decomp: Vec<u8> = Vec::new();
        lzma_rs::xz_decompress(&mut cur, &mut decomp).unwrap();
        Tar::decode(decomp)
    }
}
pub struct TarBz;
impl Decode for TarBz {
    fn decode(buf: Vec<u8>) -> Option<Files> {
        use bzip2_rs::DecoderReader;
        let cur = Cursor::new(buf);
        let reader = DecoderReader::new(cur);
        let v = reader.bytes().map(|i| i.unwrap()).collect();
        Tar::decode(v)
    }
}
use ruzstd::decoding::StreamingDecoder;

pub struct TarZstd;
impl Decode for TarZstd {
    fn decode(buf: Vec<u8>) -> Option<Files> {
        let cur = Cursor::new(buf);
        let mut decoder = StreamingDecoder::new(cur).unwrap();
        let mut result = Vec::new();
        decoder.read_to_end(&mut result).unwrap();
        Tar::decode(result)
    }
}

#[cfg(test)]
mod test {
    use crate::ty::Decode;

    use super::{Tar, TarBz, TarGz, TarXz, TarZstd};

    #[test]
    fn test_tar() {
        let buf = std::fs::read("../assets/test.tar").unwrap();
        let files = Tar::decode(buf).unwrap();
        for i in files.keys() {
            println!("name: {}", i);
            let f = files.get(&i).unwrap();
            println!("{}", f.get_buffer().len());
            // println!("{}", String::from_utf8(f.get_buffer().clone()).unwrap());
        }
    }

    #[test]
    fn test_targz() {
        let buf = std::fs::read("../assets/test.tar.gz").unwrap();
        let files = TarGz::decode(buf).unwrap();
        for i in files.keys() {
            println!("name: {}", i);
            let f = files.get(&i).unwrap();
            println!("{}", f.get_buffer().len());
            // println!("{}", String::from_utf8(f.get_buffer().clone()).unwrap());
        }
    }

    #[test]
    fn test_tarxz() {
        let buf = std::fs::read("../assets/test.tar.xz").unwrap();
        let files = TarXz::decode(buf).unwrap();
        for i in files.keys() {
            println!("name: {}", i);
            let f = files.get(&i).unwrap();
            println!("{}", f.get_buffer().len());
            // println!("{}", String::from_utf8(f.get_buffer().clone()).unwrap());
        }
    }

    #[test]
    fn test_tarzst() {
        let buf = std::fs::read("../assets/test.tar.zst").unwrap();
        let files = TarZstd::decode(buf).unwrap();
        for i in files.keys() {
            println!("name: {}", i);
            let f = files.get(&i).unwrap();
            println!("{}", f.get_buffer().len());
        }
    }

    #[test]
    fn test_tarbz() {
        let buf = std::fs::read("../assets/test.tar.bz2").unwrap();
        let files = TarBz::decode(buf).unwrap();
        for i in files.keys() {
            println!("name: {}", i);
            let f = files.get(&i).unwrap();
            println!("{}", f.get_buffer().len());
        }
    }
}
