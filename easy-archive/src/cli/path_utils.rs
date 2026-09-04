/// Path utility functions for the CLI
///
/// Provides helpers for determining output paths and generating
/// unique filenames to avoid overwriting existing files.
use crate::Fmt;

use path_clean::PathClean;
use std::path::{Path, PathBuf};

/// Generate an available (non-existing) path by appending a number suffix
///
/// If the base path already exists, this function tries appending `(1)`, `(2)`, etc.
/// until it finds a path that doesn't exist.
///
/// # Arguments
/// * `base_path` - The desired output path
/// * `is_directory` - Whether the output is a directory (affects naming)
///
/// # Returns
/// A string path that does not currently exist
pub fn get_available_path(base_path: &Path) -> String {
    upath::upath(base_path)
        .unwrap_or(base_path.to_path_buf())
        .to_string_lossy()
        .to_string()
}

/// Determine the default output path based on inputs and detected format
///
/// When no explicit output is provided, this function infers a sensible default:
/// - For decompression: derives directory name from archive filename
/// - For compression: uses the input's parent directory with `.zip` extension
/// - For multiple inputs: uses the parent directory's name
///
/// # Arguments
/// * `inputs` - List of input file/directory paths
/// * `input_fmt` - Optional detected format of the first input
///
/// # Returns
/// A default output path string
pub fn get_default_output(inputs: &[String], input_fmt: Option<Fmt>) -> String {
    if inputs.is_empty() {
        return get_available_path(&PathBuf::from("archive.zip"));
    }

    if inputs.len() == 1 {
        let input_path = Path::new(&inputs[0]).clean();

        if let Some(fmt) = input_fmt {
            let mut dir_name = input_path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "archive".to_string());

            for ext in fmt.extensions() {
                if dir_name.ends_with(ext) {
                    dir_name = dir_name[..dir_name.len() - ext.len()].to_string();
                    break;
                }
            }

            let mut base_output = input_path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(dir_name);
            if base_output.as_os_str().is_empty() {
                base_output = PathBuf::from("archive");
            }

            return get_available_path(&base_output);
        }
    }

    let first_input = Path::new(&inputs[0]).clean();
    let parent = first_input.parent().unwrap_or_else(|| Path::new(""));

    let base_name = if inputs.len() > 1 {
        parent
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "archive".to_string())
    } else {
        if first_input.is_dir() {
            first_input
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "archive".to_string())
        } else {
            let stem = first_input
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();

            if stem.is_empty() || stem.starts_with('.') {
                parent
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "archive".to_string())
            } else {
                stem
            }
        }
    };

    let final_base_name = if base_name.is_empty() {
        "archive".to_string()
    } else {
        base_name
    };
    let base_output = parent.join(format!("{}.zip", final_base_name));
    get_available_path(&base_output)
}
