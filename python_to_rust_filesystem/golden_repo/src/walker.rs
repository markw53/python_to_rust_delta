use std::fs;
use std::path::{Path, PathBuf};

pub fn walk(root: &str) -> Vec<String> {
    let root_path = Path::new(root);
    let mut out = Vec::new();

    if !root_path.exists() {
        return out;
    }

    let mut stack = vec![root_path.to_path_buf()];

    while let Some(path) = stack.pop() {
        if path != root_path {
            let rel = path
                .strip_prefix(root_path)
                .unwrap()
                .to_string_lossy()
                .to_string();
            out.push(rel);
        }

        if path.is_dir() {
            if let Ok(read) = fs::read_dir(&path) {
                for entry in read.flatten() {
                    stack.push(entry.path());
                }
            }
        }
    }

    out.sort();
    out
}
