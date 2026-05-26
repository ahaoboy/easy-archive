/// Compression handler for the CLI
///
/// Handles creating archives from input files/directories.
use crate::cli::collect::collect_files;
use crate::cli::error_display::display_error;
use crate::{Fmt, human_size};

use path_clean::PathClean;
use std::path::Path;
use std::process;

/// Handle compression operation
///
/// Collects files from the given inputs, encodes them into the target format,
/// and writes the resulting archive to the output path.
///
/// # Arguments
/// * `inputs` - List of input file/directory paths
/// * `output` - Path for the output archive
/// * `fmt` - The target archive format
pub fn handle_compression(inputs: &[String], output: &str, fmt: Fmt) {
    let mut all_files = Vec::new();
    let strip_root = inputs.len() == 1;

    for input in inputs {
        let input_path = Path::new(input).clean();
        if !input_path.exists() {
            eprintln!("Error: Input file or directory '{}' does not exist", input);
            process::exit(1);
        }

        match collect_files(&input_path, strip_root) {
            Ok(f) => all_files.extend(f),
            Err(e) => {
                eprintln!("Error: Failed to collect files from '{}': {}", input, e);
                process::exit(1);
            }
        }
    }

    let total_size: usize = all_files.iter().map(|f| f.buffer.len()).sum();
    let file_count = all_files.len();

    let buffer = match fmt.encode(all_files) {
        Ok(b) => b,
        Err(e) => {
            display_error(&e);
            process::exit(1);
        }
    };

    if let Err(e) = std::fs::write(output, &buffer) {
        eprintln!("Error: Failed to write archive '{}': {}", output, e);
        process::exit(1);
    }

    println!(
        "Compressed {} files ({}) to {} ({})",
        file_count,
        human_size(total_size),
        output,
        human_size(buffer.len()),
    );
}
