// TODO: implement testsuse filesystem_delta::walker::walk;
use std::fs;

#[test]
fn test_walk_files_and_dirs() {
    fs::create_dir("d").unwrap();
    fs::write("d/a.txt", "x").unwrap();

    let paths = walk("d");
    assert!(paths.iter().any(|p| p.ends_with("a.txt")));

    fs::remove_file("d/a.txt").unwrap();
    fs::remove_dir("d").unwrap();
}
