use easy_archive::{Fmt, human_size, mode_to_string};
use path_clean::PathClean;

const MAX_FILE_COUNT: usize = 32;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let buffer = std::fs::read(&path).expect("failed to read file");
        let fmt = Fmt::guess(&path).expect("failed to guess format");
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
        let size_max_len = info_list.iter().fold(0, |pre, cur| pre.max(cur.1.len()));
        if file_count <= MAX_FILE_COUNT {
            for (a, b, c) in info_list {
                let n = b.len();
                println!("{} {} {}", a, " ".repeat(size_max_len - n) + &b, c);
            }
        }

        if let Some(output) = std::env::args().nth(2) {
            println!("decompress to {output}");
            let path_max_len = files.iter().fold(0, |pre, cur| pre.max(cur.path.len()));
            for file in &files {
                let output_path = std::path::Path::new(&output).clean();
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
                if file_count <= MAX_FILE_COUNT {
                    println!(
                        "{} -> {}",
                        file.path.to_owned() + &" ".repeat(path_max_len - file.path.len()),
                        output_path.to_string_lossy(),
                    )
                }
            }
            if file_count > MAX_FILE_COUNT {
                println!("decompress ${file_count} files to {output}");
            }
        }
    } else {
        println!("usage:\neasy-archive <file> [dir]")
    }
}
