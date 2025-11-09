# Easy Archive

A cross-platform Rust library and CLI tool for working with various archive formats. Supports TAR, ZIP, and their compressed variants with a simple, unified API.

## Features

- 🗜️ **Multiple Formats**: TAR, TAR.GZ, TAR.XZ, TAR.BZ2, TAR.ZSTD, ZIP
- 🎯 **Modular**: Enable only the formats you need via Cargo features
- ⚡ **Operation-Specific**: Separate `encode` and `decode` features for minimal binaries
- 🚀 **Performance**: Optimized with buffered I/O and efficient compression
- 🔒 **Type-Safe**: Comprehensive error handling with structured error types
- 🌐 **Cross-Platform**: Works on Windows, macOS, and Linux
- 📦 **Small Binaries**: Optional dependencies keep your binary size minimal
- 🔧 **CLI Tool**: Command-line interface for quick archive operations

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
easy-archive = "0.2"
```

### With Specific Features Only

To minimize binary size, enable only the formats you need:

```toml
[dependencies]
easy-archive = { version = "0.2", default-features = false, features = ["tar", "tar-gz", "zip"] }
```

### As a CLI Tool

Using Cargo:

```bash
cargo install easy-archive
```

Using cargo-binstall:

```bash
cargo binstall easy-archive
```

Using npm/pnpm:

```bash
pnpm install @easy-install/easy-archive -g
```

## Quick Start

### Library Usage

```rust
use easy_archive::{Fmt, File};

// Decode an archive
let data = std::fs::read("archive.tar.gz")?;
let files = Fmt::TarGz.decode(data)?;

for file in &files {
    println!("{}: {} bytes", file.path, file.buffer.len());
}

// Encode files into an archive
let files = vec![
    File {
        path: "hello.txt".to_string(),
        buffer: b"Hello, world!".to_vec(),
        mode: Some(0o644),
        is_dir: false,
        last_modified: None,
    },
    File {
        path: "data/info.json".to_string(),
        buffer: b"{\"version\": \"1.0\"}".to_vec(),
        mode: Some(0o644),
        is_dir: false,
        last_modified: None,
    },
];

let archive = Fmt::Zip.encode(files)?;
std::fs::write("output.zip", archive)?;
```

### CLI Usage

Decompress an archive:

```bash
ea archive.tar.gz output_dir/
```

Compress a directory:

```bash
ea input_dir/ archive.tar.gz
```

## Supported Formats

| Format | Extensions | Feature Flag | Compression |
|--------|-----------|--------------|-------------|
| TAR | `.tar` | `tar` | None |
| TAR + Gzip | `.tar.gz`, `.tgz` | `tar-gz` | Gzip |
| TAR + XZ | `.tar.xz`, `.txz` | `tar-xz` | LZMA2 |
| TAR + Bzip2 | `.tar.bz2`, `.tbz2` | `tar-bz` | Bzip2 |
| TAR + Zstd | `.tar.zst`, `.tzst`, `.tzstd` | `tar-zstd` | Zstandard |
| ZIP | `.zip` | `zip` | Various |

## Feature Flags

The library uses Cargo features to enable/disable format support and operations:

### Operation Features

- `encode` - Enable archive creation (encoding)
- `decode` - Enable archive extraction (decoding)
- `default` - Enables both `encode` and `decode` with all formats

**Usage Pattern**: Combine operation features with format features to enable specific functionality.
- Use `["encode", "tar-xz"]` to enable only TAR.XZ encoding
- Use `["decode", "zip"]` to enable only ZIP decoding
- Use `["encode", "decode", "tar-gz"]` to enable both operations for TAR.GZ

### Format Features

- `tar` - Plain TAR format
- `tar-gz` - Gzip-compressed TAR (requires `tar`)
- `tar-xz` - XZ-compressed TAR (requires `tar`)
- `tar-bz` - Bzip2-compressed TAR (requires `tar`)
- `tar-zstd` - Zstd-compressed TAR (requires `tar`)
- `zip` - ZIP format

### Other Features

- `cli` - Enables CLI binary (includes all formats and operations)
- `wasm` - WebAssembly support
- `rc-zip` - Alternative ZIP implementation (optional)

### Examples

**Only ZIP decoding** (smallest binary for extraction-only use case):

```toml
easy-archive = { version = "0.2", default-features = false, features = ["decode", "zip"] }
```

**Only TAR.GZ encoding** (for creating archives only):

```toml
easy-archive = { version = "0.2", default-features = false, features = ["encode", "tar-gz"] }
```

**Only TAR.XZ decoding** (specific format extraction):

```toml
easy-archive = { version = "0.2", default-features = false, features = ["decode", "tar-xz"] }
```

**Multiple formats with decode only**:

```toml
easy-archive = { version = "0.2", default-features = false, features = ["decode", "tar-gz", "zip"] }
```

**TAR with Gzip and Zstd** (both encode and decode):

```toml
easy-archive = { version = "0.2", default-features = false, features = ["encode", "decode", "tar-gz", "tar-zstd"] }
```

**All formats, decode only**:

```toml
easy-archive = { version = "0.2", default-features = false, features = ["decode", "tar", "tar-gz", "tar-xz", "tar-bz", "tar-zstd", "zip"] }
```

## API Documentation

### Core Types

#### `Fmt` Enum

Represents archive formats:

```rust
pub enum Fmt {
    Tar,      // Plain TAR
    TarGz,    // Gzip-compressed TAR
    TarXz,    // XZ-compressed TAR
    TarBz,    // Bzip2-compressed TAR
    TarZstd,  // Zstd-compressed TAR
    Zip,      // ZIP
}
```

Methods:

- `decode(buffer: Vec<u8>) -> Result<Vec<File>>` - Decode an archive
- `encode(files: Vec<File>) -> Result<Vec<u8>>` - Encode files into an archive
- `guess(name: &str) -> Option<Fmt>` - Guess format from filename
- `extensions() -> &[&str]` - Get file extensions for this format

#### `File` Struct

Represents a file or directory in an archive:

```rust
pub struct File {
    pub buffer: Vec<u8>,           // File content
    pub path: String,              // Relative path
    pub mode: Option<u32>,         // Unix permissions (e.g., 0o755)
    pub is_dir: bool,              // Is this a directory?
    pub last_modified: Option<u64>, // Unix timestamp
}
```

#### `ArchiveError` Enum

Structured error types:

```rust
pub enum ArchiveError {
    Io(std::io::Error),
    DecodeFailed { format: String, reason: String },
    EncodeFailed { format: String, reason: String },
    DuplicateFiles { paths: Vec<String> },
    UnsupportedFormat(String),
    InvalidArchive(String),
    CompressionError(String),
    DecompressionError(String),
}
```

### Error Handling

The library uses `Result<T, ArchiveError>` for all fallible operations:

```rust
use easy_archive::{Fmt, ArchiveError};

match Fmt::TarGz.decode(data) {
    Ok(files) => {
        println!("Extracted {} files", files.len());
    }
    Err(ArchiveError::DecodeFailed { format, reason }) => {
        eprintln!("Failed to decode {}: {}", format, reason);
    }
    Err(ArchiveError::DuplicateFiles { paths }) => {
        eprintln!("Duplicate files: {:?}", paths);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Examples

### Automatic Format Detection

```rust
use easy_archive::Fmt;

let filename = "archive.tar.gz";
if let Some(fmt) = Fmt::guess(filename) {
    let data = std::fs::read(filename)?;
    let files = fmt.decode(data)?;
    println!("Extracted {} files", files.len());
}
```

### Creating a ZIP Archive

```rust
use easy_archive::{Fmt, File};

let files = vec![
    File {
        path: "README.md".to_string(),
        buffer: b"# My Project".to_vec(),
        mode: Some(0o644),
        is_dir: false,
        last_modified: Some(1234567890),
    },
    File {
        path: "src/".to_string(),
        buffer: vec![],
        mode: Some(0o755),
        is_dir: true,
        last_modified: Some(1234567890),
    },
    File {
        path: "src/main.rs".to_string(),
        buffer: b"fn main() {}".to_vec(),
        mode: Some(0o644),
        is_dir: false,
        last_modified: Some(1234567890),
    },
];

let archive = Fmt::Zip.encode(files)?;
std::fs::write("project.zip", archive)?;
```

### Extracting with Metadata

```rust
use easy_archive::Fmt;

let data = std::fs::read("archive.tar.gz")?;
let files = Fmt::TarGz.decode(data)?;

for file in files {
    println!("Path: {}", file.path);
    println!("Size: {} bytes", file.buffer.len());
    println!("Mode: {:o}", file.mode.unwrap_or(0));
    println!("Is directory: {}", file.is_dir);
    if let Some(mtime) = file.last_modified {
        println!("Modified: {}", mtime);
    }
    println!();
}
```

### Converting Between Formats

```rust
use easy_archive::Fmt;

// Read TAR.GZ
let data = std::fs::read("input.tar.gz")?;
let files = Fmt::TarGz.decode(data)?;

// Convert to ZIP
let zip_data = Fmt::Zip.encode(files)?;
std::fs::write("output.zip", zip_data)?;
```

## Binary Size Optimization

The library is designed to minimize binary size through granular feature flags:

### Size Comparison Examples

| Configuration | Approximate Binary Size | Use Case |
|--------------|------------------------|----------|
| `default` | ~2-3 MB | Full functionality |
| `["decode", "zip"]` | ~500 KB | ZIP extraction only |
| `["encode", "tar-gz"]` | ~400 KB | TAR.GZ creation only |
| `["decode", "tar-xz"]` | ~600 KB | TAR.XZ extraction only |
| `["encode", "decode", "zip"]` | ~800 KB | ZIP only (both operations) |

### Optimization Strategies

1. **Enable only needed operations**: Use `encode` OR `decode`, not both
2. **Select specific formats**: Don't enable all formats if you only need one
3. **Combine wisely**: `["decode", "tar-gz"]` is much smaller than `["default"]`

### Example: Minimal Extraction Tool

For a tool that only extracts ZIP files:

```toml
[dependencies]
easy-archive = { version = "0.2", default-features = false, features = ["decode", "zip"] }
```

This removes:
- All encoding logic
- All other format support (TAR, XZ, BZ2, ZSTD)
- Unused compression libraries

## Performance Tips

1. **Choose the Right Format**:
   - **ZIP with Zstd**: Best balance of compression and speed
   - **TAR.GZ**: Good compression, widely compatible
   - **TAR.XZ**: Best compression, slower
   - **TAR.ZSTD**: Fast compression, good ratio
   - **Plain TAR**: No compression, fastest

2. **Pre-allocate Vectors**: When creating many files, use `Vec::with_capacity()`

3. **Avoid Duplicate Checks**: The library automatically checks for duplicates during encoding

4. **Streaming**: For very large archives, consider processing files in batches

5. **Feature Selection**: Only enable the formats you need to reduce binary size and compilation time

## Error Handling Best Practices

```rust
use easy_archive::{Fmt, ArchiveError};

fn process_archive(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read(path)?;

    let fmt = Fmt::guess(path)
        .ok_or_else(|| ArchiveError::UnsupportedFormat(path.to_string()))?;

    let files = fmt.decode(data)?;

    // Process files...

    Ok(())
}
```

## Duplicate File Detection

The library automatically detects duplicate file paths during encoding:

```rust
use easy_archive::{Fmt, File, ArchiveError};

let files = vec![
    File { path: "file.txt".to_string(), ..Default::default() },
    File { path: "file.txt".to_string(), ..Default::default() }, // Duplicate!
];

match Fmt::Zip.encode(files) {
    Err(ArchiveError::DuplicateFiles { paths }) => {
        println!("Duplicate files detected: {:?}", paths);
    }
    _ => {}
}
```

## Platform-Specific Notes

### Unix Permissions

On Unix systems, file permissions are preserved:

```rust
let file = File {
    path: "script.sh".to_string(),
    buffer: b"#!/bin/bash\necho hello".to_vec(),
    mode: Some(0o755), // rwxr-xr-x
    is_dir: false,
    last_modified: None,
};
```

### Windows

On Windows, Unix permissions are ignored but the library still works correctly.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
git clone https://github.com/ahaoboy/easy-archive
cd easy-archive
cargo build
cargo test
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific features
cargo test --features tar,tar-gz
cargo test --no-default-features --features zip
```

## License

MIT License - see LICENSE file for details

## Links

- [Repository](https://github.com/ahaoboy/easy-archive)
- [Documentation](https://docs.rs/easy-archive)
- [Crates.io](https://crates.io/crates/easy-archive)

## Changelog

### Version 0.2.3

- ✨ Added feature-based format selection
- 🔧 Improved error handling with structured error types
- 🚀 Performance optimizations with buffered I/O
- 📝 Comprehensive documentation
- 🔍 Automatic duplicate file detection
- 🎯 Result-based API (breaking change from Option)
