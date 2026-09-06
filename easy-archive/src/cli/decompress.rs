/// Decompression handler for the CLI
///
/// Handles extracting archives to a target directory.
use crate::cli::error_display::display_error;
use crate::{Fmt, human_size};

use path_clean::PathClean;
use std::path::Path;
use std::process;

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
pub fn handle_decompression(input: &str, output: &str, fmt: Fmt) {
    let buffer = match std::fs::read(input) {
        Ok(buf) => buf,
        Err(e) => {
            eprintln!("Error: Failed to read input file '{}': {}", input, e);
            process::exit(1);
        }
    };

    let files = match fmt.decode(buffer) {
        Ok(f) => f,
        Err(e) => {
            display_error(&e);
            process::exit(1);
        }
    };

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

        if !dir.exists()
            && let Err(e) = std::fs::create_dir_all(dir) {
                eprintln!(
                    "Error: Failed to create directory '{}': {}",
                    dir.display(),
                    e
                );
                process::exit(1);
            }

        if file.is_dir && !output_path.exists()
            && let Err(e) = std::fs::create_dir_all(&output_path) {
                eprintln!(
                    "Error: Failed to create directory '{}': {}",
                    output_path.display(),
                    e
                );
                process::exit(1);
            }

        if !file.is_dir && !file.buffer.is_empty()
            && let Err(e) = std::fs::write(&output_path, &file.buffer) {
                eprintln!(
                    "Error: Failed to write file '{}': {}",
                    output_path.display(),
                    e
                );
                process::exit(1);
            }

        // Set permissions on Unix systems
        #[cfg(unix)]
        if let Some(mode) = file.mode {
            use std::os::unix::fs::PermissionsExt;
            if let Err(e) = std::fs::set_permissions(&output_path, std::fs::Permissions::from_mode(mode)) {
                eprintln!(
                    "Warning: Failed to set permissions for '{}': {}",
                    output_path.display(),
                    e
                );
            }
        }
    }

    println!("Decompression complete!");
}
