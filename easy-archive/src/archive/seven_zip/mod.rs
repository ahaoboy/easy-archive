/// 7z archive format
#[cfg(feature = "7z")]
pub struct SevenZip;

#[cfg(all(feature = "7z", feature = "decode"))]
mod decode;

#[cfg(all(feature = "7z", feature = "encode"))]
mod encode;
