mod archive;
mod tool;
mod ty;

pub use archive::*;
pub use tool::*;
pub use ty::*;

#[cfg(test)]
mod test {
    use crate::{File, ty::Fmt};
    use strum::IntoEnumIterator;

    #[test]
    fn test_decode() {
        for name in std::fs::read_dir("../assets").unwrap() {
            let path = name.unwrap().path();
            let buffer = std::fs::read(&path).unwrap();
            let fmt = Fmt::guess(&path.to_string_lossy()).unwrap();
            let files = fmt.decode(buffer).unwrap();
            let dist = files
                .iter()
                .find(|i| i.path == "mujs-build-0.0.11/dist-manifest.json")
                .unwrap();
            assert!(!dist.buffer.is_empty());
        }
    }

    use std::path::PathBuf;

    #[test]
    fn encode_decode() {
        for i in Fmt::iter() {
            // FIXME: support encode bz
            if i == Fmt::TarBz {
                continue;
            }
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
            let compress = i.encode(v).expect("zip error");
            println!("{:?} {}", i, compress.len());
            assert!(compress.len() > 0);
        }
    }
}
