use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use std::path::Path;

fn roundtrip(dir: &str) {
    let snap1 = create_snapshot(dir);
    let ops = compute_delta(snap1.clone(), snap1.clone());
    assert!(ops.is_empty());
}

#[test]
fn test_roundtrip_simple() {
    fs::create_dir("d").unwrap();
    fs::write("d/a.txt", "hello").unwrap();

    roundtrip("d");

    fs::remove_file("d/a.txt").unwrap();
    fs::remove_dir("d").unwrap();
}

#[test]
fn test_roundtrip_nested() {
    fs::create_dir_all("d/x/y").unwrap();
    fs::write("d/x/y/a.txt", "hello").unwrap();

    roundtrip("d");

    fs::remove_file("d/x/y/a.txt").unwrap();
    fs::remove_dir_all("d").unwrap();
}

#[test]
fn test_roundtrip_modify() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a.txt", "one").unwrap();
    fs::write("dst/a.txt", "two").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    apply_patch("src", ops).unwrap();

    let snap1 = create_snapshot("src");
    let snap2 = create_snapshot("dst");
    let ops2 = compute_delta(snap1, snap2);

    assert!(ops2.is_empty());

    fs::remove_file("src/a.txt").unwrap();
    fs::remove_file("dst/a.txt").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}

#[test]
fn test_roundtrip_symlink() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    std::os::unix::fs::symlink("t.txt", "src/link").unwrap();
    std::os::unix::fs::symlink("t.txt", "dst/link").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    assert!(ops.is_empty());

    fs::remove_file("src/link").unwrap();
    fs::remove_file("dst/link").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}

#[test]
fn test_roundtrip_idempotent() {
    fs::create_dir("d").unwrap();
    fs::write("d/a.txt", "hello").unwrap();

    let snap1 = create_snapshot("d");
    let ops = compute_delta(snap1.clone(), snap1.clone());
    assert!(ops.is_empty());

    apply_patch("d", ops).unwrap();

    let snap2 = create_snapshot("d");
    assert_eq!(snap1.entries.len(), snap2.entries.len());

    fs::remove_file("d/a.txt").unwrap();
    fs::remove_dir("d").unwrap();
}
