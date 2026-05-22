use std::path::Path;

pub fn normalize_path<P: AsRef<Path>>(p: P) -> String {
    let p = p.as_ref().to_string_lossy();

    let mut parts = Vec::new();

    for part in p.split('/') {
        match part {
            "" | "." => continue,
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }

    parts.join("/")
}
