use easy_archive::{File, Fmt};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn guess(name: String) -> Option<Fmt> {
    Fmt::guess(&name)
}

#[wasm_bindgen]
pub fn extensions(fmt: Fmt) -> Vec<String> {
    fmt.extensions().iter().map(|i| i.to_string()).collect()
}

#[wasm_bindgen]
pub fn decode(fmt: Fmt, buffer: Vec<u8>) -> Option<Vec<File>> {
    fmt.decode(buffer).ok()
}

#[wasm_bindgen]
pub fn encode(fmt: Fmt, files: Vec<File>) -> Option<Vec<u8>> {
    fmt.encode(files).ok()
}
