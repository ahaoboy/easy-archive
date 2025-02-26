mod archive;
mod tool;
mod ty;

pub use archive::*;
pub use tool::*;
pub use ty::*;

#[cfg(test)]
mod test {
    use crate::ty::Fmt;

    #[test]
    fn test_decode() {
        for name in std::fs::read_dir("../assets").unwrap() {
            let path = name.unwrap().path();
            let buffer = std::fs::read(&path).unwrap();
            let fmt = Fmt::guess(&path.to_string_lossy()).unwrap();
            let files = fmt.decode(buffer).unwrap();
            let dist = files
                .iter()
                .find(|i| i.path == "mujs-build-0.0.11/dist-manifest.json")
                .unwrap();
            assert!(!dist.buffer.is_empty());
        }
    }
}
