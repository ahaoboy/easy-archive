/// Command-line interface for easy-archive
///
/// This binary provides a simple CLI for compressing and decompressing archives.
use easy_archive::{ArchiveError, Fmt, human_size};

#[cfg(feature = "encode")]
use easy_archive::File;

use path_clean::PathClean;
use std::fs;

#[cfg(feature = "encode")]
use std::io;

use std::path::Path;
use std::process;

/// Collect files and directories recursively, skipping symlinks
///
/// # Arguments
/// * `input_path` - The path to collect files from
///
/// # Returns
/// * `Ok(Vec<File>)` - List of collected files
/// * `Err(io::Error)` - If file system operations fail
#[cfg(feature = "encode")]
fn collect_files(input_path: &Path) -> io::Result<Vec<File>> {
    let mut files = Vec::new();
    let input_path = input_path.clean();

    // If input is a file, process it directly
    if input_path.is_file() {
        let buffer = fs::read(&input_path)?;
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
        collect_files_recursive(&input_path, &input_path, &mut files)?;
    }

    Ok(files)
}

/// Recursive helper function to collect files and directories
///
/// # Arguments
/// * `base_path` - The base path for calculating relative paths
/// * `current_path` - The current directory being processed
/// * `files` - Mutable vector to accumulate files
#[cfg(feature = "encode")]
fn collect_files_recursive(
    base_path: &Path,
    current_path: &Path,
    files: &mut Vec<File>,
) -> io::Result<()> {
    for entry in fs::read_dir(current_path)? {
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
            let buffer = fs::read(&path)?;
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

/// Display user-friendly error message
fn display_error(error: &ArchiveError) {
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

/// Handle decompression operation
#[cfg(feature = "decode")]
fn handle_decompression(input: &str, output: &str, fmt: Fmt) {
    let buffer = match fs::read(input) {
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
            && let Err(e) = fs::create_dir_all(dir)
        {
            eprintln!(
                "Error: Failed to create directory '{}': {}",
                dir.display(),
                e
            );
            process::exit(1);
        }

        if file.is_dir
            && !output_path.exists()
            && let Err(e) = fs::create_dir_all(&output_path)
        {
            eprintln!(
                "Error: Failed to create directory '{}': {}",
                output_path.display(),
                e
            );
            process::exit(1);
        }

        if !file.is_dir
            && let Err(e) = fs::write(&output_path, &file.buffer)
        {
            eprintln!(
                "Error: Failed to write file '{}': {}",
                output_path.display(),
                e
            );
            process::exit(1);
        }

        // Set permissions on Unix systems
        #[cfg(not(windows))]
        if let Some(mode) = file.mode {
            use std::os::unix::fs::PermissionsExt;
            if let Err(e) = fs::set_permissions(&output_path, fs::Permissions::from_mode(mode)) {
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

/// Handle compression operation
#[cfg(feature = "encode")]
fn handle_compression(input: &str, output: &str, fmt: Fmt) {
    let input_path = Path::new(input).clean();
    if !input_path.exists() {
        eprintln!("Error: Input file or directory '{}' does not exist", input);
        process::exit(1);
    }

    let files = match collect_files(&input_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Failed to collect files: {}", e);
            process::exit(1);
        }
    };

    let total_size: usize = files.iter().map(|f| f.buffer.len()).sum();
    let file_count = files.len();

    let buffer = match fmt.encode(files) {
        Ok(b) => b,
        Err(e) => {
            display_error(&e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(output, &buffer) {
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

fn print_usage() {
    println!("Usage: ea <input> <output>");
    println!("\nExamples:");

    #[cfg(feature = "decode")]
    println!("  ea archive.tar.gz output_dir/    # Decompress");

    #[cfg(feature = "encode")]
    println!("  ea input_dir/ archive.tar.gz     # Compress");

    println!("\nSupported formats:");
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

    if formats.is_empty() {
        println!("  (No formats enabled)");
    } else {
        println!("  {}", formats.join(", "));
    }

    println!("\nEnabled operations:");
    #[cfg(feature = "decode")]
    println!("  - Decode (extract archives)");
    #[cfg(feature = "encode")]
    println!("  - Encode (create archives)");

    #[cfg(not(any(feature = "decode", feature = "encode")))]
    println!("  (No operations enabled)");
}

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next();
    let output = args.next();

    if input.is_none() || output.is_none() {
        print_usage();
        process::exit(1);
    }

    let input = input.unwrap();
    let output = output.unwrap();

    // Detect input and output format
    let input_fmt = Fmt::guess(&input);
    let output_fmt = Fmt::guess(&output);

    // Handle compression or decompression based on enabled features
    match (input_fmt, output_fmt) {
        #[cfg(feature = "decode")]
        (Some(fmt), None) => {
            // Decompression
            handle_decompression(&input, &output, fmt);
        }
        #[cfg(feature = "encode")]
        (None, Some(fmt)) => {
            // Compression
            handle_compression(&input, &output, fmt);
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Both input and output are archive formats.");
            eprintln!("Please specify one as a directory for compression/decompression.");
            process::exit(1);
        }
        #[cfg(all(feature = "decode", not(feature = "encode")))]
        (None, Some(_)) => {
            eprintln!("Error: Encode operation is not enabled.");
            eprintln!("This binary was compiled with decode-only support.");
            process::exit(1);
        }
        #[cfg(all(not(feature = "decode"), feature = "encode"))]
        (Some(_), None) => {
            eprintln!("Error: Decode operation is not enabled.");
            eprintln!("This binary was compiled with encode-only support.");
            process::exit(1);
        }
        (None, None) => {
            #[cfg(all(not(feature = "decode"), not(feature = "encode")))]
            {
                eprintln!("Error: No operations enabled.");
                eprintln!("Please enable 'encode' or 'decode' feature.");
            }
            #[cfg(any(feature = "decode", feature = "encode"))]
            {
                eprintln!("Error: Cannot identify input and output formats.");
                eprintln!("At least one must be a recognized archive format.");
                print_usage();
            }
            process::exit(1);
        }
    }
}
