/// Error display utilities for the CLI
///
/// Provides user-friendly error message formatting for the command-line interface.
use crate::ArchiveError;

/// Display a user-friendly error message
///
/// Prints the error to stderr and provides additional context for specific
/// error types such as duplicate files or unsupported formats.
///
/// # Arguments
/// * `error` - The archive error to display
pub fn display_error(error: &ArchiveError) {
    eprintln!("Error: {}", error);

    // Provide additional context for specific error types
    match error {
        #[cfg(feature = "encode")]
        ArchiveError::DuplicateFiles { paths } => {
            eprintln!("\nDuplicate file paths detected:");
            for path in paths {
                eprintln!("  - {}", path);
            }
            eprintln!("\nPlease ensure all file paths are unique.");
        }
        ArchiveError::UnsupportedFormat(fmt) => {
            eprintln!("\nThe format '{}' is not supported or not enabled.", fmt);
            eprintln!("Check that the corresponding feature flag is enabled.");
        }
        _ => {}
    }
}
