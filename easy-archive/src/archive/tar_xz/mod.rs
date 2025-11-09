/// XZ-compressed TAR archive format
#[cfg(feature = "tar-xz")]
pub struct TarXz;

#[cfg(all(feature = "tar-xz", feature = "decode"))]
mod decode;

#[cfg(all(feature = "tar-xz", feature = "encode"))]
mod encode;
