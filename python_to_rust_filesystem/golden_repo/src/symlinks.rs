use std::fs;
use std::path::Path;

pub fn read_symlink(path: &str) -> Option<String> {
    let p = Path::new(path);
    if p.is_symlink() {
        fs::read_link(p)
            .ok()
            .map(|t| t.to_string_lossy().to_string())
    } else {
        None
    }
}
