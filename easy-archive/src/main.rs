use easy_archive::{Fmt, human_size, mode_to_string};
use path_clean::PathClean;
use std::fs::{self};
use std::io;
use std::path::Path;

// Function to collect files and directories recursively, skipping symlinks
fn collect_files(input_path: &Path) -> io::Result<Vec<easy_archive::File>> {
    let mut files = Vec::new();
    let input_path = input_path.clean();

    // If input is a file, process it directly
    if input_path.is_file() {
        let buffer = fs::read(&input_path)?;
        let file_name = input_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();
        files.push(easy_archive::File {
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

// Recursive helper function to collect files and directories
fn collect_files_recursive(
    base_path: &Path,
    current_path: &Path,
    files: &mut Vec<easy_archive::File>,
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
            files.push(easy_archive::File {
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
            files.push(easy_archive::File {
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

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next();
    let output = args.next();

    if input.is_none() || output.is_none() {
        println!("usage:\nea <input> <output>");
        println!("input and output parameters are required");
        return;
    }

    let input = input.unwrap();
    let output = output.unwrap();

    // Detect input and output format
    let input_fmt = Fmt::guess(&input);
    let output_fmt = Fmt::guess(&output);

    // Handle compression or decompression
    match (input_fmt, output_fmt) {
        (Some(fmt), None) => {
            // Decompression
            let buffer = std::fs::read(&input).expect("failed to read input file");
            let files = fmt.decode(buffer).expect("failed to decode");
            let mut info_list = vec![];
            let mut total_size = 0;
            let file_count = files.len();
            for file in &files {
                let size = file.buffer.len();
                info_list.push((
                    mode_to_string(file.mode.unwrap_or(0), file.is_dir),
                    human_size(size),
                    &file.path,
                ));
                total_size += size;
            }
            println!("{} of {} files", human_size(total_size), file_count);
            println!("decompress to {output}");
            for file in &files {
                let output_path = Path::new(&output).clean();
                let output_path = output_path.join(&file.path).clean();
                let dir = output_path.parent().expect("failed to get parent dir");
                if !dir.exists() {
                    std::fs::create_dir_all(dir).expect("failed to create dir");
                }
                if file.is_dir && !output_path.exists() {
                    std::fs::create_dir_all(&output_path).expect("failed to create dir");
                }

                let buffer = &file.buffer;
                if !file.is_dir {
                    std::fs::write(&output_path, buffer).expect("failed to write file");
                }

                #[cfg(not(windows))]
                if let Some(mode) = file.mode {
                    std::fs::set_permissions(
                        &output_path,
                        std::os::unix::fs::PermissionsExt::from_mode(mode),
                    )
                    .expect("failed to set permissions");
                }

            }
        }
        (None, Some(fmt)) => {
            // Compression
            let input_path = Path::new(&input).clean();
            if !input_path.exists() {
                println!("input file or directory does not exist");
                return;
            }

            let files = collect_files(&input_path).expect("failed to collect files");
            let total_size: usize = files.iter().map(|f| f.buffer.len()).sum();
            let file_count = files.len();
            let buffer = fmt.encode(files).expect("failed to encode files");
            std::fs::write(&output, &buffer).expect("failed to write archive");
            println!(
                "compressed {} files({}) to {}({})",
                file_count,
                human_size(total_size),
                output,
                human_size(buffer.len()),
            );
        }
        (Some(_), Some(_)) => {
            println!("both input and output are archive formats, please choose one as a directory");
        }
        (None, None) => {
            println!(
                "cannot identify input and output formats, at least one must be an archive format"
            );
        }
    }
}
