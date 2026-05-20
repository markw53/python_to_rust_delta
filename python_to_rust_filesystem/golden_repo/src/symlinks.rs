use std::fs;
use std::path::Path;

pub fn read_symlink(path: &Path) -> Option<String> {
    match fs::read_link(path) {
        Ok(target) => Some(target.to_string_lossy().to_string()),
        Err(_) => None,
    }
}
