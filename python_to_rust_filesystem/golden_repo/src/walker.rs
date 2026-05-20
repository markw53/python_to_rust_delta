use walkdir::WalkDir;
use std::path::PathBuf;

pub fn walk(root: &str) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .collect()
}
