/// Archive format conversion (transcoding)
///
/// Provides functionality to convert between different archive formats
/// by decoding from one format and encoding to another.
///
/// This module is only available when both `encode` and `decode` features
/// are enabled, plus the `convert` feature.
use crate::{ArchiveError, Fmt, Result};

/// Convert an archive from one format to another
///
/// Decodes the input archive and re-encodes the extracted files into
/// the target format. This is more efficient than extracting to disk
/// and re-compressing, as all operations happen in memory.
///
/// # Arguments
/// * `input` - Raw bytes of the input archive
/// * `input_fmt` - The format of the input archive
/// * `output_fmt` - The desired output format
///
/// # Returns
/// * `Ok(Vec<u8>)` - The converted archive data
/// * `Err(ArchiveError)` - If decoding or encoding fails
///
/// # Example
/// ```no_run
/// use easy_archive::{Fmt, convert::convert_archive};
///
/// let tar_gz_data = std::fs::read("archive.tar.gz")?;
/// let zip_data = convert_archive(tar_gz_data, Fmt::TarGz, Fmt::Zip)?;
/// std::fs::write("archive.zip", zip_data)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn convert_archive(
    input: Vec<u8>,
    input_fmt: Fmt,
    output_fmt: Fmt,
) -> Result<Vec<u8>> {
    let files = input_fmt.decode(input)?;
    output_fmt.encode(files)
}

/// Convert an archive from one format to another by reading from a file
/// and writing the result to another file.
///
/// # Arguments
/// * `input_path` - Path to the input archive file
/// * `output_path` - Path for the output archive file
/// * `input_fmt` - The format of the input archive
/// * `output_fmt` - The desired output format
///
/// # Returns
/// * `Ok(())` - If conversion succeeds
/// * `Err(ArchiveError)` - If reading, decoding, encoding, or writing fails
///
/// # Example
/// ```no_run
/// use easy_archive::{Fmt, convert::convert_archive_file};
///
/// convert_archive_file("archive.tar.gz", "archive.zip", Fmt::TarGz, Fmt::Zip)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn convert_archive_file(
    input_path: &str,
    output_path: &str,
    input_fmt: Fmt,
    output_fmt: Fmt,
) -> Result<()> {
    let input = std::fs::read(input_path).map_err(|e| {
        ArchiveError::Io(e)
    })?;
    let output = convert_archive(input, input_fmt, output_fmt)?;
    std::fs::write(output_path, output).map_err(|e| {
        ArchiveError::Io(e)
    })
}
