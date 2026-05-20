use filesystem_delta::metadata::{extract_mode, extract_mtime};
use std::fs;

#[test]
fn test_extract_metadata_file() {
    let path = "meta.txt";
    fs::write(path, "x").unwrap();

    let meta = fs::metadata(path).unwrap();
    assert!(extract_mode(&meta).is_some());
    assert!(extract_mtime(&meta).is_some());

    fs::remove_file(path).unwrap();
}

