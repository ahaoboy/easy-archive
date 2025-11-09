/// Plain TAR archive format
#[cfg(feature = "tar")]
pub struct Tar;

#[cfg(all(feature = "tar", feature = "decode"))]
pub(crate) mod decode;

#[cfg(all(feature = "tar", feature = "encode"))]
pub(crate) mod encode;
