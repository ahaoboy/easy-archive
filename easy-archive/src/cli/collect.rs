/// File collection utilities for the CLI
///
/// Provides functions to recursively collect files and directories
/// for archive creation.
use crate::File;

use path_clean::PathClean;
use std::io;
use std::path::Path;

/// Collect files and directories recursively, skipping symlinks
///
/// If the input is a file, it's processed directly. If it's a directory,
/// all files and subdirectories are collected recursively.
///
/// # Arguments
/// * `input_path` - The path to collect files from
/// * `strip_root` - If true, the root directory name is stripped from paths
///
/// # Returns
/// * `Ok(Vec<File>)` - List of collected files
/// * `Err(io::Error)` - If file system operations fail
pub fn collect_files(input_path: &Path, strip_root: bool) -> io::Result<Vec<File>> {
    let mut files = Vec::new();
    let input_path = input_path.clean();

    // If input is a file, process it directly
    if input_path.is_file() {
        let buffer = std::fs::read(&input_path)?;
        let file_name = input_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();
        files.push(File {
            path: file_name,
            buffer,
            is_dir: false,
            mode: None,
            last_modified: None,
        });
        return Ok(files);
    }

    // If input is a directory, process recursively
    if input_path.is_dir() {
        let base_path = if strip_root {
            input_path.as_path()
        } else {
            input_path.parent().unwrap_or_else(|| Path::new(""))
        };

        if !strip_root {
            let rel_path = input_path
                .strip_prefix(base_path)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| {
                    input_path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default()
                });

            if !rel_path.is_empty() && rel_path != "." {
                files.push(File {
                    path: rel_path.clone(),
                    buffer: vec![],
                    is_dir: true,
                    mode: None,
                    last_modified: None,
                });
            }
        }

        collect_files_recursive(base_path, &input_path, &mut files)?;
    }

    Ok(files)
}

/// Recursive helper function to collect files and directories
///
/// # Arguments
/// * `base_path` - The base path for calculating relative paths
/// * `current_path` - The current directory being processed
/// * `files` - Mutable vector to accumulate files
pub fn collect_files_recursive(
    base_path: &Path,
    current_path: &Path,
    files: &mut Vec<File>,
) -> io::Result<()> {
    for entry in std::fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a file or directory (e.g., symlinks)
        if !path.is_file() && !path.is_dir() {
            continue;
        }

        let rel_path = path
            .strip_prefix(base_path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.to_string_lossy().to_string());

        if path.is_dir() {
            files.push(File {
                path: rel_path.clone(),
                buffer: vec![],
                is_dir: true,
                mode: None,
                last_modified: None,
            });
            // Recurse into subdirectory
            collect_files_recursive(base_path, &path, files)?;
        } else if path.is_file() {
            let buffer = std::fs::read(&path)?;
            files.push(File {
                path: rel_path,
                buffer,
                is_dir: false,
                mode: None,
                last_modified: None,
            });
        }
    }
    Ok(())
}
