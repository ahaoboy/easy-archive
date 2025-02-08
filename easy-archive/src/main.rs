use easy_archive::ty::Fmt;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let buffer = std::fs::read(&path).expect("failed to read file");
        let fmt = Fmt::guess(&path).expect("failed to guess format");
        let files = fmt.decode(buffer).expect("failed to decode");
        for (path, file) in &files {
            println!("{}: {}", path, file.get_buffer().len())
        }
        if let Some(output) = std::env::args().nth(2) {
            println!("decompress to {}", output);
            for (path, file) in &files {
                let output_path = std::path::Path::new(&output);
                let output_path = output_path.join(path);
                let dir = output_path.parent().unwrap();
                if !dir.exists() {
                    std::fs::create_dir_all(dir).unwrap();
                }
                let buffer = file.get_buffer();
                if !buffer.is_empty() {
                    std::fs::write(&output_path, &buffer).unwrap();
                }
                println!("{} -> {}", path, output_path.to_string_lossy(),)
            }
        }
    }
}
