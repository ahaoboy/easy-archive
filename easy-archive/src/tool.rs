pub fn clean(s: &str) -> String {
    path_clean::clean(s)
        .to_string_lossy()
        .to_string()
        .replace("\\", "/")
}

pub fn mode_to_string(mode: u32, is_dir: bool) -> String {
    let rwx_mapping = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];
    let owner = rwx_mapping[((mode >> 6) & 0b111) as usize];
    let group = rwx_mapping[((mode >> 3) & 0b111) as usize];
    let others = rwx_mapping[(mode & 0b111) as usize];
    let d = if is_dir { "d" } else { "-" };
    format!("{d}{owner}{group}{others}")
}

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
