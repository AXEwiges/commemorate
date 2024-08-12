use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{CommemorateError, CommemorateResult};

pub fn ensure_commemorate_dir() -> CommemorateResult<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        CommemorateError::DirectoryCreationError("Unable to determine home directory".to_string())
    })?;
    let commemorate_dir = home_dir.join(".commemorate");
    if !commemorate_dir.exists() {
        fs::create_dir(&commemorate_dir).map_err(|e| {
            CommemorateError::DirectoryCreationError(format!(
                "Failed to create .commemorate directory: {}",
                e
            ))
        })?;
    }
    Ok(commemorate_dir)
}

pub fn read_file(path: &Path) -> CommemorateResult<Vec<u8>> {
    std::fs::read(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => CommemorateError::MemoryNotFound,
        std::io::ErrorKind::PermissionDenied => CommemorateError::MemoryAccessError,
        _ => CommemorateError::FileReadError(e),
    })
}

pub fn write_file(path: &Path, contents: &[u8]) -> CommemorateResult<()> {
    std::fs::write(path, contents).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => CommemorateError::MemoryNotFound,
        std::io::ErrorKind::PermissionDenied => CommemorateError::MemoryAccessError,
        _ => CommemorateError::FileReadError(e),
    })?;
    Ok(())
}

pub fn list_memoria_files() -> CommemorateResult<Vec<PathBuf>> {
    let commemorate_dir = ensure_commemorate_dir()?;
    let entries = fs::read_dir(commemorate_dir).map_err(CommemorateError::FileReadError)?;
    let mut memoria_files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(CommemorateError::FileReadError)?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "memoria") {
            memoria_files.push(path);
        }
    }

    memoria_files.sort_by(|a, b| {
        b.metadata()
            .and_then(|m| m.modified())
            .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH)
            .cmp(
                &a.metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH),
            )
    });

    Ok(memoria_files)
}

pub fn get_memoria_path(name: &str) -> CommemorateResult<PathBuf> {
    let commemorate_dir = ensure_commemorate_dir()?;
    Ok(commemorate_dir.join(format!("{}.memoria", name)))
}
