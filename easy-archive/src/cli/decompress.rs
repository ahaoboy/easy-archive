/// Decompression handler for the CLI
///
/// Handles extracting archives to a target directory.
use crate::{
    Fmt,
    error::{ArchiveError, Result},
    human_size,
};

use path_clean::PathClean;
use std::path::Path;

/// Create a symbolic link, without assuming anything about the platform.
#[cfg(unix)]
fn make_symlink(target: &str, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

/// Windows has no untyped link: pick file or directory by looking at the
/// target. This requires developer mode or the symlink privilege.
#[cfg(windows)]
fn make_symlink(target: &str, link: &Path) -> std::io::Result<()> {
    let resolved = link.parent().unwrap_or_else(|| Path::new("")).join(target);
    if resolved.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}

#[cfg(not(any(unix, windows)))]
fn make_symlink(_target: &str, _link: &Path) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "symbolic links are not supported on this platform",
    ))
}

/// Whether a link would stay inside the extraction root.
///
/// Archives are untrusted input: a link pointing outside (`../../etc`) can be
/// used to redirect the files that follow it, so those links are never
/// created. The link target itself is untouched in the library API; this is a
/// policy of the CLI only.
fn link_stays_inside(root: &Path, link: &Path, target: &str) -> bool {
    let target_path = Path::new(target);
    if target_path.is_absolute() {
        return false;
    }
    let dir = link.parent().unwrap_or_else(|| Path::new(""));
    dir.join(target_path).clean().starts_with(root.clean())
}

/// Write a symlink entry as a real symlink.
///
/// Where creating a link is not permitted (Windows without developer mode),
/// the target is copied instead so the extracted tree stays usable; the
/// warning makes the difference visible.
fn write_link_entry(root: &Path, link: &Path, target: &str) -> Result<()> {
    if !link_stays_inside(root, link, target) {
        eprintln!(
            "Warning: skipped symlink '{}' -> '{target}' because it points outside the output directory",
            link.display()
        );
        return Ok(());
    }

    // An existing entry would make link creation fail (and `write` never
    // replaces a link, it follows it).
    if std::fs::symlink_metadata(link).is_ok() {
        let _ = std::fs::remove_file(link);
    }

    let err = match make_symlink(target, link) {
        Ok(()) => return Ok(()),
        Err(err) => err,
    };

    let source = link.parent().unwrap_or_else(|| Path::new("")).join(target);
    std::fs::copy(&source, link).map_err(|_| {
        ArchiveError::io_context(
            err,
            format!(
                "failed to create symlink '{}' -> '{target}'",
                link.display()
            ),
        )
    })?;
    eprintln!(
        "Warning: created a copy of '{target}' at '{}' because symlinks are not permitted here",
        link.display()
    );
    Ok(())
}

/// Write a single decoded entry to `output_path`, creating parent
/// directories as needed and applying Unix permissions when available.
///
/// `root` is the extraction directory; symlinks aiming outside of it are
/// skipped.
fn write_file_entry(root: &Path, output_path: &Path, file: &crate::File) -> Result<()> {
    let output_path = output_path.clean();
    if let Some(dir) = output_path.parent()
        && !dir.as_os_str().is_empty()
        && !dir.exists()
    {
        std::fs::create_dir_all(dir).map_err(|e| {
            ArchiveError::io_context(e, format!("failed to create directory '{}'", dir.display()))
        })?;
    }

    if file.is_dir {
        if !output_path.exists() {
            std::fs::create_dir_all(&output_path).map_err(|e| {
                ArchiveError::io_context(
                    e,
                    format!("failed to create directory '{}'", output_path.display()),
                )
            })?;
        }
        return Ok(());
    }

    if let Some(target) = &file.link_target {
        return write_link_entry(root, &output_path, target);
    }

    std::fs::write(&output_path, &file.buffer).map_err(|e| {
        ArchiveError::io_context(
            e,
            format!("failed to write file '{}'", output_path.display()),
        )
    })?;

    #[cfg(unix)]
    if let Some(mode) = file.mode {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) =
            std::fs::set_permissions(&output_path, std::fs::Permissions::from_mode(mode))
        {
            eprintln!(
                "Warning: Failed to set permissions for '{}': {}",
                output_path.display(),
                e
            );
        }
    }

    Ok(())
}

/// Handle decompression operation
///
/// Reads an archive file, decodes its contents, and extracts all files
/// to the specified output directory. Handles directory creation and
/// sets Unix permissions where applicable.
///
/// # Arguments
/// * `input` - Path to the input archive file
/// * `output` - Path to the output directory
/// * `fmt` - The format of the input archive
///
/// # Errors
/// Returns an [`ArchiveError`] if the archive cannot be read or decoded,
/// or if extracting an entry to disk fails.
pub fn handle_decompression(input: &str, output: &str, fmt: Fmt) -> Result<()> {
    let buffer = std::fs::read(input).map_err(|e| {
        ArchiveError::io_context(e, format!("failed to read input file '{}'", input))
    })?;

    let files = fmt.decode(buffer)?;

    let mut total_size = 0;
    let file_count = files.len();

    for file in &files {
        total_size += file.buffer.len();
    }

    println!("{} of {} files", human_size(total_size), file_count);
    println!("Decompressing to {}", output);

    // Single-file formats (e.g. plain gzip) hold exactly one entry with no
    // directory structure, so write it directly to the output path rather
    // than treating `output` as a directory.
    if fmt.is_single_file() {
        if let Some(file) = files.first() {
            write_file_entry(Path::new(""), Path::new(output), file)?;
        }
        println!("Decompression complete!");
        return Ok(());
    }

    let root = Path::new(output).clean();
    for file in &files {
        let output_path = root.join(&file.path).clean();
        write_file_entry(&root, &output_path, file)?;
    }

    println!("Decompression complete!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::link_stays_inside;
    use std::path::Path;

    /// Links that resolve inside the extraction root are kept; anything that
    /// leaves it (or is absolute) is refused.
    #[test]
    fn only_links_inside_the_root_are_created() {
        let root = Path::new("/tmp/out");
        for (link, target, expected) in [
            ("/tmp/out/lib/libfoo.so.1", "libfoo.so.1.2.3", true),
            ("/tmp/out/docs/libdir", "../lib", true),
            ("/tmp/out/pack/escape", "../../secret", false),
            ("/tmp/out/abs", "/etc", false),
            ("/tmp/out/deep/a", "../../../etc/passwd", false),
        ] {
            assert_eq!(
                link_stays_inside(root, Path::new(link), target),
                expected,
                "{link} -> {target}"
            );
        }
    }
}
