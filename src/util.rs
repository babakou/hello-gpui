use std::fs;
use std::io;
use std::path::Path;

fn collect_matching_files(
    root: &Path,
    current: &Path,
    extension: &str,
    rows: &mut Vec<(String, u64)>,
) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            collect_matching_files(root, &path, extension, rows)?;
        } else if file_type.is_file() {
            let matches = path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case(extension));

            if matches {
                let relative_path = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");

                let size = entry.metadata()?.len();
                rows.push((relative_path, size));
            }
        }
    }

    Ok(())
}

pub fn scan_files(root: &Path, extension: &str) -> io::Result<Vec<(String, u64)>> {
    if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Folder Path does not point to a directory",
        ));
    }

    let extension = extension.trim().trim_start_matches('.');
    if extension.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Extension is empty",
        ));
    }

    let mut rows = Vec::new();
    collect_matching_files(root, root, extension, &mut rows)?;

    Ok(rows)
}
