use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;

#[test]
fn test_unicode_paths() {
    fs::create_dir("d").unwrap();
    fs::write("d/こんにちは.txt", "hello").unwrap();

    let snap = create_snapshot("d");
    assert!(snap.entries.iter().any(|e| e.path.contains("こんにちは")));

    fs::remove_file("d/こんにちは.txt").unwrap();
    fs::remove_dir("d").unwrap();
}

#[test]
fn test_unicode_nested() {
    fs::create_dir_all("d/привет/世界").unwrap();
    fs::write("d/привет/世界/a.txt", "x").unwrap();

    let snap = create_snapshot("d");
    assert!(snap.entries.iter().any(|e| e.path.contains("привет")));
    assert!(snap.entries.iter().any(|e| e.path.contains("世界")));

    fs::remove_file("d/привет/世界/a.txt").unwrap();
    fs::remove_dir_all("d").unwrap();
}
