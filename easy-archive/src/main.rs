use easy_archive::{
    tool::{human_size, mode_to_string},
    ty::Fmt,
};
use path_clean::PathClean;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let buffer = std::fs::read(&path).expect("failed to read file");
        let fmt = Fmt::guess(&path).expect("failed to guess format");
        let files = fmt.decode(buffer).expect("failed to decode");
        let mut info_list = vec![];
        for (path, file) in &files {
            info_list.push((
                mode_to_string(file.mode.unwrap_or(0), file.is_dir),
                human_size(file.buffer.len()),
                path,
            ));
        }
        let size_max_len = info_list.iter().fold(0, |pre, cur| pre.max(cur.1.len()));
        for (a, b, c) in info_list {
            let n = b.len();
            println!("{} {} {}", a, " ".repeat(size_max_len - n) + &b, c);
        }
        if let Some(output) = std::env::args().nth(2) {
            println!("decompress to {}", output);
            for (path, file) in &files {
                let output_path = std::path::Path::new(&output).clean();
                let output_path = output_path.join(path).clean();
                let dir = output_path.parent().expect("failed to get parent dir");
                if !dir.exists() {
                    std::fs::create_dir_all(dir).expect("failed to create dir");
                }
                if file.is_dir && !output_path.exists() {
                    std::fs::create_dir_all(&output_path).expect("failed to create dir");
                }

                let buffer = &file.buffer;
                if !buffer.is_empty() && !file.is_dir {
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
