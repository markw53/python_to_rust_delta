use filesystem_delta::paths::normalize_path;
use std::path::Path;

#[test]
fn test_normalize_simple() {
    let p = Path::new("a/b/c");
    assert_eq!(normalize_path(p), "a/b/c");
}

#[test]
fn test_normalize_backslashes() {
    let p = Path::new("a\\b\\c");
    assert_eq!(normalize_path(p), "a/b/c");
}

