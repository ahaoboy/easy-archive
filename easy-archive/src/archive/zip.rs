use crate::ty::{Decode, File, Files};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use zip::ZipArchive;

pub struct Zip;

impl Decode for Zip {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        let mut c = Cursor::new(Vec::new());
        c.write_all(&buffer).ok()?;
        c.seek(SeekFrom::Start(0)).ok()?;
        let mut files = Files::new();
        let mut archive = ZipArchive::new(c).ok()?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).ok()?;
            if file.is_file() {
                let mut buffer = vec![];
                file.read_to_end(&mut buffer).ok()?;
                let name = file.name();
                files.insert(
                    name.to_string(),
                    File::new(name.to_string(), buffer.clone(), None),
                );
            }
        }
        Some(files)
    }
}
