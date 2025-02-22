use easy_archive::ty::{File, Fmt};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn guess(name: String) -> Option<Fmt> {
    Fmt::guess(&name)
}

#[wasm_bindgen]
pub fn extensions(fmt: Fmt) -> Vec<String> {
    fmt.extensions()
}

#[wasm_bindgen]
pub fn decode(fmt: Fmt, buffer: Vec<u8>) -> Option<Vec<File>> {
    fmt.decode(buffer)
}
