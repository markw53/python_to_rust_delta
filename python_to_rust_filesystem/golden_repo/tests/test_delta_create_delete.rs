use filesystem_delta::{create_snapshot, compute_delta};
use std::fs;
use std::path::Path;

#[test]
fn test_create_file() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();
    fs::write("dst/a.txt", "x").unwrap();

    let src_snap = create_snapshot("src");
    let dst_snap = create_snapshot("dst");

    let ops = compute_delta(src_snap, dst_snap);
    assert!(ops.iter().any(|o| o.op == "create_file" && o.path == "a.txt"));
}

