use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;

#[test]
fn test_symlink_create() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    std::os::unix::fs::symlink("t.txt", "dst/link").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert!(ops.iter().any(|o| o.op == "symlink" && o.path == "link"));

    fs::remove_file("dst/link").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}

#[test]
fn test_symlink_delete() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    std::os::unix::fs::symlink("t.txt", "src/link").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert!(ops
        .iter()
        .any(|o| o.op == "delete_file" && o.path == "link"));

    fs::remove_file("src/link").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}

#[test]
fn test_symlink_target_change() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    std::os::unix::fs::symlink("a.txt", "src/link").unwrap();
    std::os::unix::fs::symlink("b.txt", "dst/link").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert!(ops
        .iter()
        .any(|o| o.op == "symlink" && o.path == "link" && o.target.as_deref() == Some("b.txt")));

    fs::remove_file("src/link").unwrap();
    fs::remove_file("dst/link").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}
