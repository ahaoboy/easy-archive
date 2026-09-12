/// Compression handler for the CLI
///
/// Handles creating archives from input files/directories.
use crate::cli::collect::collect_files;
use crate::{
    Fmt,
    error::{ArchiveError, Result},
    human_size,
};

use path_clean::PathClean;
use std::path::Path;

/// Handle compression operation
///
/// Collects files from the given inputs, encodes them into the target format,
/// and writes the resulting archive to the output path.
///
/// # Arguments
/// * `inputs` - List of input file/directory paths
/// * `output` - Path for the output archive
/// * `fmt` - The target archive format
///
/// # Errors
/// Returns an [`ArchiveError`] if an input does not exist, files cannot be
/// collected, encoding fails, or the archive cannot be written.
pub fn handle_compression(inputs: &[String], output: &str, fmt: Fmt) -> Result<()> {
    let mut all_files = Vec::new();
    let strip_root = inputs.len() == 1;

    for input in inputs {
        let input_path = Path::new(input).clean();
        if !input_path.exists() {
            return Err(ArchiveError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("input file or directory '{}' does not exist", input),
            )));
        }

        let files = collect_files(&input_path, strip_root).map_err(|e| {
            ArchiveError::io_context(e, format!("failed to collect files from '{}'", input))
        })?;
        all_files.extend(files);
    }

    let total_size: usize = all_files.iter().map(|f| f.buffer.len()).sum();
    let file_count = all_files.len();

    let buffer = fmt.encode(all_files)?;

    std::fs::write(output, &buffer).map_err(|e| {
        ArchiveError::io_context(e, format!("failed to write archive '{}'", output))
    })?;

    println!(
        "Compressed {} files ({}) to {} ({})",
        file_count,
        human_size(total_size),
        output,
        human_size(buffer.len()),
    );

    Ok(())
}
