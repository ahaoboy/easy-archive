use crate::archive::{
    tar::{Tar, TarBz, TarGz, TarXz, TarZstd},
    zip::Zip,
};

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fmt {
    Tar,
    TarGz,
    TarXz,
    TarBz,
    TarZstd,
    Zip,
}

impl Fmt {
    pub fn decode(&self, buffer: Vec<u8>) -> Option<Vec<File>> {
        match self {
            Fmt::Zip => Zip::decode(buffer),
            Fmt::Tar => Tar::decode(buffer),
            Fmt::TarGz => TarGz::decode(buffer),
            Fmt::TarXz => TarXz::decode(buffer),
            Fmt::TarBz => TarBz::decode(buffer),
            Fmt::TarZstd => TarZstd::decode(buffer),
        }
    }

    pub fn guess(name: &str) -> Option<Self> {
        use Fmt::*;
        for fmt in [Tar, TarGz, TarXz, TarBz, TarZstd, Zip] {
            for ext in fmt.extensions() {
                if name.ends_with(&ext) {
                    return Some(fmt);
                }
            }
        }
        None
    }

    pub fn extensions(&self) -> Vec<String> {
        match self {
            Fmt::Tar => vec![".tar".to_string()],
            Fmt::TarGz => vec![".tar.gz".to_string(), ".tgz".to_string()],
            Fmt::TarXz => vec![".tar.xz".to_string(), ".txz".to_string()],
            Fmt::TarBz => vec![".tar.bz2".to_string(), ".tbz2".to_string()],
            Fmt::TarZstd => vec![
                ".tzstd".to_string(),
                ".tzst".to_string(),
                ".tar.zst".to_string(),
            ],
            Fmt::Zip => vec![".zip".to_string()],
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Default)]
pub struct File {
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub buffer: Vec<u8>,
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub path: String,
    pub mode: Option<u32>,
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "isDir"))]
    pub is_dir: bool,
}

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl File {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(path: String, buffer: Vec<u8>, mode: Option<u32>, is_dir: bool) -> Self {
        File {
            path,
            buffer,
            mode,
            is_dir,
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl File {
    // To reduce memory consumption
    // get buffer should be placed last
    #[wasm_bindgen(getter = buffer)]
    pub fn get_buffer(self) -> Vec<u8> {
        self.buffer
    }

    #[wasm_bindgen(getter = path)]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

pub trait Encode {
    fn encode(files: Vec<File>) -> Option<Vec<u8>>;
}

pub trait Decode {
    fn decode(buffer: Vec<u8>) -> Option<Vec<File>>;
}

pub trait Archive: Encode + Decode {}

#[cfg(test)]
mod test {
    use super::Fmt;

    #[test]
    fn test_guess() {
        for (name, fmt) in [
            ("a.zip", Fmt::Zip),
            ("a.tar", Fmt::Tar),
            ("a.tar.gz", Fmt::TarGz),
            ("a.tar.xz", Fmt::TarXz),
            ("a.tar.bz2", Fmt::TarBz),
        ] {
            assert!(Fmt::guess(name) == Some(fmt))
        }
    }
}
