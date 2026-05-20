use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;

#[test]
fn test_apply_create_file() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();
    fs::write("dst/a.txt", "x").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    apply_patch("src", ops).unwrap();

    assert!(fs::metadata("src/a.txt").unwrap().is_file());
}

#[test]
fn test_apply_delete_file() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();
    fs::write("src/a.txt", "x").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    apply_patch("src", ops).unwrap();

    assert!(!Path::new("src/a.txt").exists());
}
