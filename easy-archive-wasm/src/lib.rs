use easy_archive::ty::{Files, Fmt};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn decode(name: String, buffer: Vec<u8>) -> Option<Files> {
    let fmt = Fmt::guess(&name)?;
    fmt.decode(buffer)
}
