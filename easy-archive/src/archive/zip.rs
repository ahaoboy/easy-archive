use crate::{Decode, Encode, File, tool::clean};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use time::OffsetDateTime;
use zip::DateTime;

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
            let last_modified = file
                .last_modified()
                .and_then(|i| OffsetDateTime::try_from(i).ok())
                .map(|i| i.unix_timestamp() as u64);

            files.push(File::new(path, buffer.clone(), None, is_dir, last_modified));
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

impl Encode for Zip {
    fn encode(files: Vec<File>) -> Option<Vec<u8>> {
        use std::collections::HashSet;
        use std::io::prelude::*;
        use zip::write::SimpleFileOptions;

        let mut v = vec![];
        let mut c = std::io::Cursor::new(&mut v);
        let mut zip = zip::ZipWriter::new(&mut c);
        let mut dir_set = HashSet::new();

        for i in files.iter().filter(|i| i.is_dir) {
            dir_set.insert(i.path.clone());
            let mut options = SimpleFileOptions::default();
            if let Some(last) = i.last_modified {
                let mod_time = OffsetDateTime::from_unix_timestamp(last as i64)
                    .ok()
                    .and_then(|i| DateTime::try_from(i).ok());
                if let Some(offset) = mod_time {
                    options = options.last_modified_time(offset);
                }
            }
            zip.add_directory(i.path.as_str(), options).ok()?;
        }

        for i in files.iter().filter(|i| !i.is_dir) {
            if let Some(p) = std::path::Path::new(&i.path).parent() {
                let path = p.to_string_lossy().to_string();
                if dir_set.contains(&path) {
                    continue;
                }

                dir_set.insert(i.path.clone());
                let mut options = SimpleFileOptions::default();
                if let Some(last) = i.last_modified {
                    let mod_time = OffsetDateTime::from_unix_timestamp(last as i64)
                        .ok()
                        .and_then(|i| DateTime::try_from(i).ok());
                    if let Some(offset) = mod_time {
                        options = options.last_modified_time(offset);
                    }
                }
                zip.add_directory(i.path.as_str(), options).ok()?;
                dir_set.insert(path);
            }
        }

        for i in &files {
            if i.is_dir {
                continue;
            }
            let mode = i.mode.unwrap_or(0o755);
            let mut options = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(mode);

            if let Some(last) = i.last_modified {
                let mod_time = OffsetDateTime::from_unix_timestamp(last as i64)
                    .ok()
                    .and_then(|i| DateTime::try_from(i).ok());
                if let Some(offset) = mod_time {
                    options = options.last_modified_time(offset);
                }
            }

            zip.start_file(i.path.as_str(), options).ok()?;
            zip.write_all(&i.buffer).ok()?;
        }
        zip.finish().ok()?;
        Some(v)
    }
}
