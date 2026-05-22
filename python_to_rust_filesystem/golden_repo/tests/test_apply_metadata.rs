use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[test]
fn test_apply_chmod() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a.txt", "x").unwrap();
    fs::write("dst/a.txt", "x").unwrap();

    let mut perms = fs::metadata("dst/a.txt").unwrap().permissions();
    perms.set_mode(0o777);
    fs::set_permissions("dst/a.txt", perms).unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    apply_patch("src", ops).unwrap();

    let mode = fs::metadata("src/a.txt").unwrap().permissions().mode();
    assert_eq!(mode & 0o777, 0o777);
}
