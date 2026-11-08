use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

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
            let metadata = entry.metadata()?;

            if !is_locally_available(&metadata) {
                continue;
            }

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

                let size = metadata.len();
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

pub fn compress_and_delete_files(
    seven_zip: &Path,
    root: &Path,
    relative_files: &[String],
    delete_after_compress: bool,
) -> anyhow::Result<()> {
    let seven_zip = seven_zip.canonicalize()?;
    anyhow::ensure!(seven_zip.is_file(), "7-Zip executable not found");

    let root = root.canonicalize()?;

    for relative_file in relative_files {
        let relative_path = Path::new(relative_file);
        anyhow::ensure!(
            !relative_path.is_absolute(),
            "Expected a relative file path: {relative_file}"
        );

        let source = root.join(relative_path).canonicalize()?;
        anyhow::ensure!(
            source.starts_with(&root) && source.is_file(),
            "File is outside the root folder or is not a file: {relative_file}"
        );

        let relative_source = source.strip_prefix(&root)?;

        // 例: photo.jpg → photo.jpg.7z（元ファイルの隣に作成）
        let mut archive_name = source
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid file name: {relative_file}"))?
            .to_os_string();
        archive_name.push(".7z");
        let archive = source.with_file_name(archive_name);

        anyhow::ensure!(
            !archive.exists(),
            "Archive already exists: {}",
            archive.display()
        );

        let mut options = Vec::from(["a", "-t7z"]);
        if delete_after_compress {
            options.push("-sdel");
        }

        let status = Command::new(&seven_zip)
            .current_dir(&root)
            .args(options)
            .arg(&archive)
            .arg(relative_source)
            .status()?;

        anyhow::ensure!(
            status.success(),
            "7-Zip compression failed: {relative_file}"
        );
    }

    Ok(())
}

#[cfg(windows)]
fn is_locally_available(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_OFFLINE: u32 = 0x0000_1000;
    const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 0x0004_0000;
    const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x0040_0000;

    let unavailable_attributes = FILE_ATTRIBUTE_OFFLINE
        | FILE_ATTRIBUTE_RECALL_ON_OPEN
        | FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS;

    metadata.file_attributes() & unavailable_attributes == 0
}

#[cfg(not(windows))]
fn is_locally_available(_metadata: &fs::Metadata) -> bool {
    true
}
