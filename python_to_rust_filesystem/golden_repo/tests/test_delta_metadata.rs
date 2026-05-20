use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn test_chmod_change() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a.txt", "x").unwrap();
    fs::write("dst/a.txt", "x").unwrap();

    let mut perms = fs::metadata("dst/a.txt").unwrap().permissions();
    perms.set_mode(0o777);
    fs::set_permissions("dst/a.txt", perms).unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert!(ops.iter().any(|o| o.op == "chmod" && o.path == "a.txt"));

    fs::remove_file("src/a.txt").unwrap();
    fs::remove_file("dst/a.txt").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}

#[test]
fn test_utimes_change() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a.txt", "x").unwrap();
    fs::write("dst/a.txt", "x").unwrap();

    // Force mtime difference
    let meta = fs::metadata("dst/a.txt").unwrap();
    let mut perms = meta.permissions();
    perms.set_mode(meta.permissions().mode());
    fs::set_permissions("dst/a.txt", perms).unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert!(ops.iter().any(|o| o.op == "utimes" && o.path == "a.txt"));

    fs::remove_file("src/a.txt").unwrap();
    fs::remove_file("dst/a.txt").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}
