pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::archive::{
    tar::{Tar, TarBz, TarGz, TarXz, TarZstd},
    zip::Zip,
};

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
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
    pub fn encode(&self, files: Vec<File>) -> Option<Vec<u8>> {
        match self {
            Fmt::Zip => Zip::encode(files),
            Fmt::Tar => Tar::encode(files),
            Fmt::TarGz => TarGz::encode(files),
            Fmt::TarXz => TarXz::encode(files),
            Fmt::TarBz => TarBz::encode(files),
            Fmt::TarZstd => TarZstd::encode(files),
        }
    }

    pub fn guess(name: &str) -> Option<Self> {
        for fmt in Fmt::iter() {
            for ext in fmt.extensions() {
                if name.ends_with(ext) {
                    return Some(fmt);
                }
            }
        }
        None
    }

    pub fn extensions(&self) -> &[&'static str] {
        match self {
            Fmt::Tar => &[".tar"],
            Fmt::TarGz => &[".tar.gz", ".tgz"],
            Fmt::TarXz => &[".tar.xz", ".txz"],
            Fmt::TarBz => &[".tar.bz2", ".tbz2"],
            Fmt::TarZstd => &[".tzstd", ".tzst", ".tar.zst"],
            Fmt::Zip => &[".zip"],
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
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "lastModified"))]
    pub last_modified: Option<u64>,
}

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl File {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(
        path: String,
        buffer: Vec<u8>,
        mode: Option<u32>,
        is_dir: bool,
        last_modified: Option<u64>,
    ) -> Self {
        File {
            path,
            buffer,
            mode,
            is_dir,
            last_modified,
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
    #[wasm_bindgen(setter = buffer)]
    pub fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    #[wasm_bindgen(getter = path)]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    #[wasm_bindgen(setter = path)]
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    #[wasm_bindgen(getter = bufferSize)]
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
    #[wasm_bindgen]
    pub fn clone(&self) -> Self {
        Clone::clone(self)
    }
}

pub trait Encode {
    fn encode(files: Vec<File>) -> Option<Vec<u8>>;
}

pub trait Decode {
    fn decode<T: AsRef<[u8]>>(buffer: T) -> Option<Vec<File>>;
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
