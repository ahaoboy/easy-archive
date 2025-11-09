/// Bzip2-compressed TAR archive format
#[cfg(feature = "tar-bz")]
pub struct TarBz;

#[cfg(all(feature = "tar-bz", feature = "decode"))]
mod decode;

#[cfg(all(feature = "tar-bz", feature = "encode"))]
mod encode;
