use crate::{
    tool::clean,
    ty::{Decode, File},
};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

pub struct Zip;

#[cfg(feature = "zip")]
fn decode_zip(buffer: &[u8]) -> Option<Vec<File>> {
    let mut c = Cursor::new(Vec::new());
    c.write_all(buffer).ok()?;
    c.seek(SeekFrom::Start(0)).ok()?;
    let mut files = Vec::new();
    let mut archive = zip::ZipArchive::new(c).ok()?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).ok()?;
        if file.is_file() {
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).ok()?;
            let path = file.name();
            let is_dir = file.is_dir() || path.ends_with("/");
            let path = clean(path);
            files.push(File::new(path, buffer.clone(), None, is_dir));
        }
    }
    Some(files)
}

#[cfg(feature = "rc-zip")]
fn decode_rc_zip(buffer: &[u8]) -> Option<Vec<File>> {
    use rc_zip_sync::ReadZip;
    let reader = buffer.read_zip().ok()?;
    let mut files = Vec::new();
    for entry in reader.entries() {
        let path = entry.name.clone();
        let buffer = entry.bytes().ok()?;
        let mode = entry.mode.0;
        let is_dir = matches!(entry.kind(), rc_zip::parse::EntryKind::Directory);
        let path = clean(&path);
        files.push(File {
            buffer,
            path,
            mode: Some(mode),
            is_dir,
        });
    }
    Some(files)
}

impl Decode for Zip {
    fn decode(buffer: Vec<u8>) -> Option<Vec<File>> {
        #[cfg(feature = "zip")]
        return decode_zip(&buffer);
        #[cfg(feature = "rc-zip")]
        return decode_rc_zip(&buffer);
    }
}
