use easy_archive::ty::Fmt;
use path_clean::PathClean;

fn mode_to_string(mode: u32, is_dir: bool) -> String {
    if mode > 0o777 {
        panic!("Invalid mode: must be in range 0 to 0o777");
    }

    let rwx_mapping = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];

    let owner = rwx_mapping[((mode >> 6) & 0b111) as usize]; // Owner permissions
    let group = rwx_mapping[((mode >> 3) & 0b111) as usize]; // Group permissions
    let others = rwx_mapping[(mode & 0b111) as usize]; // Others permissions
    let d = if is_dir { "d" } else { "-" };
    format!("{}{}{}{}", d, owner, group, others)
}

fn hunman_size(bytes: usize) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size = bytes as f64;
    let mut index = 0;

    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, units[index])
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let buffer = std::fs::read(&path).expect("failed to read file");
        let fmt = Fmt::guess(&path).expect("failed to guess format");
        let files = fmt.decode(buffer).expect("failed to decode");
        let mut info_list = vec![];
        for (path, file) in &files {
            info_list.push((
                mode_to_string(file.mode.unwrap_or(0), file.is_dir()),
                hunman_size(file.buffer.len()),
                path,
            ));
        }
        let size_max_len = info_list.iter().fold(0, |pre, cur| pre.max(cur.1.len()));
        for (a, b, c) in info_list {
            let n = b.len();
            println!("{} {} {}", a, b + &" ".repeat(size_max_len - n), c);
        }
        if let Some(output) = std::env::args().nth(2) {
            println!("decompress to {}", output);
            for (path, file) in &files {
                println!("path {:?} {:?}", path, file.mode);
                let output_path = std::path::Path::new(&output).clean();
                let output_path = output_path.join(path).clean();
                let dir = output_path.parent().expect("failed to get parent dir");
                if !dir.exists() {
                    std::fs::create_dir_all(dir).expect("failed to create dir");
                }
                if file.is_dir() && !output_path.exists() {
                    println!("output_path {:?}", output_path);
                    std::fs::create_dir_all(&output_path).expect("failed to create dir");
                }

                let buffer = &file.buffer;
                if !buffer.is_empty() && !file.is_dir() {
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

                println!("{} -> {}", path, output_path.to_string_lossy(),)
            }
        }
    } else {
        println!("usage:\neasy-archive <file> [dir]")
    }
}
