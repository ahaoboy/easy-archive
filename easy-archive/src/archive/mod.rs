#[cfg(feature = "tar")]
pub mod tar;

#[cfg(feature = "tar-gz")]
pub mod tar_gz;

#[cfg(feature = "tar-xz")]
pub mod tar_xz;

#[cfg(feature = "tar-bz")]
pub mod tar_bz;

#[cfg(feature = "tar-zstd")]
pub mod tar_zstd;

#[cfg(feature = "zip")]
pub mod zip;

#[cfg(feature = "tar")]
pub use tar::Tar;

#[cfg(feature = "tar-gz")]
pub use tar_gz::TarGz;

#[cfg(feature = "tar-xz")]
pub use tar_xz::TarXz;

#[cfg(feature = "tar-bz")]
pub use tar_bz::TarBz;

#[cfg(feature = "tar-zstd")]
pub use tar_zstd::TarZstd;

#[cfg(feature = "zip")]
pub use zip::Zip;
