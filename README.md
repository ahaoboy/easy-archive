## install

```bash
cargo binstall easy-archive
```

```bash
pnpm install @easy-install/easy-archive -g
```

## usage

The CLI supports multiple inputs and optional output paths. If the output path (`-o`) is omitted, it intelligently infers a default output filename (or extraction folder) and avoids overwriting by appending incremental numbers.

```bash
# compress a single directory or file (smart output: ./test.zip)
# Note: Single directory compression will strip the root folder name in the zip
easy-archive test

# compress multiple files into a specific archive
easy-archive dir1 file2.txt -o archive.zip

# compress multiple directories using auto-inferred name
easy-archive dir1 dir2

# decompress an archive (smart extraction to: ./test/)
easy-archive test.zip

# decompress into a specific folder
easy-archive test.zip -o ./test
```

## symlinks

TAR (all variants) and ZIP store symbolic links as links instead of copying or dropping them: `File::link_target` holds the target, extraction recreates the link (falling back to a copy where the platform forbids links), and packing records the link again. 7z and gzip have no portable link representation, so encoding rejects them instead of writing an empty file.

## web

https://ahaoboy.github.io/easy-archive/