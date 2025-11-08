use crate::File;
/// Utility functions for archive operations
use crate::error::{ArchiveError, Result};
use std::collections::HashSet;

/// Clean and normalize a file path
///
/// Converts backslashes to forward slashes and removes redundant path components
///
/// # Arguments
/// * `s` - The path string to clean
///
/// # Returns
/// A normalized path string with forward slashes
#[inline]
pub fn clean(s: &str) -> String {
    path_clean::clean(s)
        .to_string_lossy()
        .to_string()
        .replace("\\", "/")
}

/// Convert Unix file mode to a human-readable permission string
///
/// # Arguments
/// * `mode` - Unix file mode (e.g., 0o755)
/// * `is_dir` - Whether the entry is a directory
///
/// # Returns
/// A string representation like "drwxr-xr-x" or "-rw-r--r--"
///
/// # Example
/// ```
/// use easy_archive::mode_to_string;
/// assert_eq!(mode_to_string(0o755, true), "drwxr-xr-x");
/// assert_eq!(mode_to_string(0o644, false), "-rw-r--r--");
/// ```
#[inline]
pub fn mode_to_string(mode: u32, is_dir: bool) -> String {
    let rwx_mapping = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];
    let owner = rwx_mapping[((mode >> 6) & 0b111) as usize];
    let group = rwx_mapping[((mode >> 3) & 0b111) as usize];
    let others = rwx_mapping[(mode & 0b111) as usize];
    let d = if is_dir { "d" } else { "-" };
    format!("{d}{owner}{group}{others}")
}

/// Round a floating point number to one decimal place, removing trailing zeros
#[inline]
fn round(value: f64) -> String {
    let mut s = format!("{value:.1}");
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    s
}

/// Convert byte size to human-readable format
///
/// # Arguments
/// * `bytes` - The size in bytes
///
/// # Returns
/// A human-readable string like "1.5M" or "256K"
///
/// # Example
/// ```
/// use easy_archive::human_size;
/// assert_eq!(human_size(0), "0");
/// assert_eq!(human_size(1024), "1K");
/// assert_eq!(human_size(1536), "1.5K");
/// ```
#[inline]
pub fn human_size(bytes: usize) -> String {
    if bytes == 0 {
        return "0".to_string();
    }
    let units = ["", "K", "M", "G", "T", "P", "E", "Z", "Y"];
    let b = bytes as f64;
    let exponent = (b.log(1024.0)).floor() as usize;
    let value = b / 1024f64.powi(exponent as i32);
    let rounded = round(value);
    format!("{}{}", rounded, units[exponent])
}

/// Check for duplicate file paths in a list of files
///
/// # Arguments
/// * `files` - Slice of File entries to check
///
/// # Returns
/// * `Ok(())` if no duplicates found
/// * `Err(ArchiveError::DuplicateFiles)` if duplicates are detected
///
/// # Example
/// ```
/// use easy_archive::{File, check_duplicate_files};
///
/// let files = vec![
///     File { path: "a.txt".to_string(), ..Default::default() },
///     File { path: "b.txt".to_string(), ..Default::default() },
/// ];
/// assert!(check_duplicate_files(&files).is_ok());
///
/// let files_with_dup = vec![
///     File { path: "a.txt".to_string(), ..Default::default() },
///     File { path: "a.txt".to_string(), ..Default::default() },
/// ];
/// assert!(check_duplicate_files(&files_with_dup).is_err());
/// ```
pub fn check_duplicate_files(files: &[File]) -> Result<()> {
    let mut seen = HashSet::with_capacity(files.len());
    let mut duplicates = Vec::new();

    for file in files {
        if !seen.insert(&file.path) {
            duplicates.push(file.path.clone());
        }
    }

    if !duplicates.is_empty() {
        return Err(ArchiveError::DuplicateFiles { paths: duplicates });
    }

    Ok(())
}
