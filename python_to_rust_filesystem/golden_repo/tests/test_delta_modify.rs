use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;

#[test]
fn test_modify_file_hash_change() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a.txt", "one").unwrap();
    fs::write("dst/a.txt", "two").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert!(ops
        .iter()
        .any(|o| o.op == "modify_file" && o.path == "a.txt"));

    fs::remove_file("src/a.txt").unwrap();
    fs::remove_file("dst/a.txt").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}

#[test]
fn test_modify_file_no_change() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a.txt", "same").unwrap();
    fs::write("dst/a.txt", "same").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert!(ops.is_empty());

    fs::remove_file("src/a.txt").unwrap();
    fs::remove_file("dst/a.txt").unwrap();
    fs::remove_dir("src").unwrap();
    fs::remove_dir("dst").unwrap();
}
