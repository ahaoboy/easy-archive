/// Decompression handler for the CLI
///
/// Handles extracting archives to a target directory.
use crate::{
    Fmt,
    error::{ArchiveError, Result},
    human_size,
};

use path_clean::PathClean;
use std::path::Path;

/// Handle decompression operation
///
/// Reads an archive file, decodes its contents, and extracts all files
/// to the specified output directory. Handles directory creation and
/// sets Unix permissions where applicable.
///
/// # Arguments
/// * `input` - Path to the input archive file
/// * `output` - Path to the output directory
/// * `fmt` - The format of the input archive
///
/// # Errors
/// Returns an [`ArchiveError`] if the archive cannot be read or decoded,
/// or if extracting an entry to disk fails.
pub fn handle_decompression(input: &str, output: &str, fmt: Fmt) -> Result<()> {
    let buffer = std::fs::read(input).map_err(|e| {
        ArchiveError::io_context(e, format!("failed to read input file '{}'", input))
    })?;

    let files = fmt.decode(buffer)?;

    let mut total_size = 0;
    let file_count = files.len();

    for file in &files {
        total_size += file.buffer.len();
    }

    println!("{} of {} files", human_size(total_size), file_count);
    println!("Decompressing to {}", output);

    for file in &files {
        let output_path = Path::new(output).clean();
        let output_path = output_path.join(&file.path).clean();
        let dir = output_path
            .parent()
            .expect("Failed to get parent directory");

        if !dir.exists() {
            std::fs::create_dir_all(dir).map_err(|e| {
                ArchiveError::io_context(
                    e,
                    format!("failed to create directory '{}'", dir.display()),
                )
            })?;
        }

        if file.is_dir && !output_path.exists() {
            std::fs::create_dir_all(&output_path).map_err(|e| {
                ArchiveError::io_context(
                    e,
                    format!("failed to create directory '{}'", output_path.display()),
                )
            })?;
        }

        if !file.is_dir && !file.buffer.is_empty() {
            std::fs::write(&output_path, &file.buffer).map_err(|e| {
                ArchiveError::io_context(
                    e,
                    format!("failed to write file '{}'", output_path.display()),
                )
            })?;
        }

        // Set permissions on Unix systems
        #[cfg(unix)]
        if let Some(mode) = file.mode {
            use std::os::unix::fs::PermissionsExt;
            if let Err(e) =
                std::fs::set_permissions(&output_path, std::fs::Permissions::from_mode(mode))
            {
                eprintln!(
                    "Warning: Failed to set permissions for '{}': {}",
                    output_path.display(),
                    e
                );
            }
        }
    }

    println!("Decompression complete!");

    Ok(())
}
