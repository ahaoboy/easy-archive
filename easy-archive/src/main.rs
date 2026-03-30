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

use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;

/// Command-line interface for easy-archive
///
/// This binary provides a simple CLI for compressing and decompressing archives.
fn get_help_text() -> &'static str {
    static HELP_TEXT: std::sync::OnceLock<String> = std::sync::OnceLock::new();

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

            #[cfg(not(any(feature = "decode", feature = "encode")))]
            help.push_str("  (No operations enabled)\n");

            help
        })
        .as_str()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help = get_help_text())]
struct Cli {
    /// Input files or directories (multiple allowed)
    #[arg(required = true)]
    inputs: Vec<String>,

    /// Output archive or directory
    #[arg(short, long)]
    output: Option<String>,
}

/// Collect files and directories recursively, skipping symlinks
///
/// # Arguments
/// * `input_path` - The path to collect files from
///
/// # Returns
/// * `Ok(Vec<File>)` - List of collected files
/// * `Err(io::Error)` - If file system operations fail
#[cfg(feature = "encode")]
fn collect_files(input_path: &Path, strip_root: bool) -> io::Result<Vec<File>> {
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
            && !file.buffer.is_empty()
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
fn handle_compression(inputs: &[String], output: &str, fmt: Fmt) {
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

fn get_available_path(base_path: &Path, is_directory: bool) -> String {
    if !base_path.exists() {
        return base_path.to_string_lossy().into_owned();
    }

    let mut i = 1;
    let parent = base_path.parent().unwrap_or_else(|| Path::new(""));
    let file_name = base_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let ext = base_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let orig_name = base_path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    loop {
        let new_name = if is_directory || ext.is_empty() {
            format!("{}({})", orig_name, i)
        } else {
            format!("{}({}).{}", file_name, i, ext)
        };

        let new_path = parent.join(&new_name);
        if !new_path.exists() {
            return new_path.to_string_lossy().into_owned();
        }
        i += 1;
    }
}

fn get_default_output(inputs: &[String], input_fmt: Option<Fmt>) -> String {
    if inputs.is_empty() {
        return get_available_path(&PathBuf::from("archive.zip"), false);
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

            return get_available_path(&base_output, true);
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
    get_available_path(&base_output, false)
}

fn main() {
    let cli = Cli::parse();
    let inputs = cli.inputs;

    let input_fmt = if inputs.len() == 1 {
        Fmt::guess(&inputs[0])
    } else {
        None // Multiple files always evaluate to a single compression output archive
    };

    let output = cli
        .output
        .unwrap_or_else(|| get_default_output(&inputs, input_fmt));

    let output_fmt = Fmt::guess(&output);

    // Handle compression or decompression based on enabled features
    match (input_fmt, output_fmt) {
        #[cfg(feature = "decode")]
        (Some(fmt), None) => {
            // Decompression
            handle_decompression(&inputs[0], &output, fmt);
        }
        #[cfg(feature = "encode")]
        (None, Some(fmt)) => {
            // Compression
            handle_compression(&inputs, &output, fmt);
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
            }
            process::exit(1);
        }
    }
}
