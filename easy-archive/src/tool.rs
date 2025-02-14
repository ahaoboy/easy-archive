pub fn clean(s: &str) -> String {
    path_clean::clean(s)
        .to_string_lossy()
        .to_string()
        .replace("\\", "/")
}
