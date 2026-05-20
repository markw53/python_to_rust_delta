use std::path::{Path, PathBuf};

pub fn normalize_path(path: &Path) -> String {
    let mut p = PathBuf::new();

    for part in path.components() {
        p.push(part.as_os_str());
    }

    p.to_string_lossy().replace('\\', "/")
}
