/// ZIP archive format
#[cfg(feature = "zip")]
pub struct Zip;

#[cfg(all(feature = "zip", feature = "decode"))]
mod decode;

#[cfg(all(feature = "zip", feature = "encode"))]
mod encode;
