use crate::ty::{Decode, File, Files};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use zip::ZipArchive;

pub struct Zip;

fn decode_zip(buffer: &[u8]) -> Option<Files> {
    let mut c = Cursor::new(Vec::new());
    c.write_all(buffer).ok()?;
    c.seek(SeekFrom::Start(0)).ok()?;
    let mut files = Files::new();
    let mut archive = ZipArchive::new(c).ok()?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).ok()?;
        if file.is_file() {
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).ok()?;
            let name = file.name();
            let is_dir = name.ends_with("/");
            files.insert(
                name.to_string(),
                File::new(name.to_string(), buffer.clone(), None, is_dir),
            );
        }
    }
    Some(files)
}

#[cfg(any(test, feature = "rc-zip"))]
fn decode_rc_zip(buffer: &[u8]) -> Option<Files> {
    use rc_zip_sync::ReadZip;
    let reader = buffer.read_zip().ok()?;
    let mut files = Files::new();
    for entry in reader.entries() {
        let path = entry.name.clone();
        let buffer = entry.bytes().unwrap();
        let mode = entry.mode.0;
        let is_dir = matches!(entry.kind(), rc_zip::parse::EntryKind::Directory);
        files.insert(
            path.clone(),
            File {
                buffer,
                path,
                mode: Some(mode),
                is_dir,
            },
        );
    }
    Some(files)
}

impl Decode for Zip {
    fn decode(buffer: Vec<u8>) -> Option<Files> {
        let files = decode_zip(&buffer);

        #[cfg(feature = "rc-zip")]
        if files.is_none() {
            return decode_rc_zip(&buffer);
        }
        files
    }
}
