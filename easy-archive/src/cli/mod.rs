//! CLI utility module for easy-archive
//!
//! Provides helper functions for the command-line interface including
//! compression/decompression handlers, file collection, error display,
//! and path utilities.

pub mod collect;
pub mod compress;
pub mod decompress;
pub mod error_display;
pub mod path_utils;

pub use collect::{collect_files, collect_files_recursive};
pub use compress::handle_compression;
pub use decompress::handle_decompression;
pub use error_display::display_error;
pub use path_utils::{get_available_path, get_default_output};

/// Get the help text for supported formats and operations
pub fn get_help_text() -> &'static str {
    use std::sync::OnceLock;

    static HELP_TEXT: OnceLock<String> = OnceLock::new();

    HELP_TEXT
        .get_or_init(|| {
            let mut help = String::new();
            help.push_str("Supported formats:\n");
            let mut formats = Vec::new();

            #[cfg(feature = "tar")]
            formats.push(".tar");
            #[cfg(feature = "tar-gz")]
            formats.extend(&[".tar.gz", ".tgz"]);
            #[cfg(feature = "tar-xz")]
            formats.extend(&[".tar.xz", ".txz"]);
            #[cfg(feature = "tar-bz")]
            formats.extend(&[".tar.bz2", ".tbz2", ".tbz"]);
            #[cfg(feature = "tar-zstd")]
            formats.extend(&[".tar.zst", ".tzst"]);
            #[cfg(feature = "zip")]
            formats.push(".zip");
            #[cfg(feature = "7z")]
            formats.push(".7z");

            if formats.is_empty() {
                help.push_str("  (No formats enabled)\n");
            } else {
                help.push_str(&format!("  {}\n", formats.join(", ")));
            }

            help.push_str("\nEnabled operations:\n");
            #[cfg(feature = "decode")]
            help.push_str("  - Decode (extract archives)\n");
            #[cfg(feature = "encode")]
            help.push_str("  - Encode (create archives)\n");
            #[cfg(all(feature = "encode", feature = "decode"))]
            help.push_str("  - Convert (transcode between formats)\n");

            #[cfg(not(any(feature = "decode", feature = "encode")))]
            help.push_str("  (No operations enabled)\n");

            help
        })
        .as_str()
}
