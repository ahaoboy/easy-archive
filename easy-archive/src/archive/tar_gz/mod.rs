/// Gzip-compressed TAR archive format
#[cfg(feature = "tar-gz")]
pub struct TarGz;

#[cfg(all(feature = "tar-gz", feature = "decode"))]
mod decode;

#[cfg(all(feature = "tar-gz", feature = "encode"))]
mod encode;
