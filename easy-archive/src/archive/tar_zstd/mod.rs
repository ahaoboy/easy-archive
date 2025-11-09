/// Zstd-compressed TAR archive format
#[cfg(feature = "tar-zstd")]
pub struct TarZstd;

#[cfg(all(feature = "tar-zstd", feature = "decode"))]
mod decode;

#[cfg(all(feature = "tar-zstd", feature = "encode"))]
mod encode;
