use filesystem_delta::symlinks::read_symlink;
use std::os::unix::fs::symlink;
use std::fs;

#[test]
fn test_read_symlink() {
    fs::write("real.txt", "x").unwrap();
    symlink("real.txt", "link").unwrap();

    let target = read_symlink("link".as_ref()).unwrap();
    assert_eq!(target, "real.txt");

    fs::remove_file("real.txt").unwrap();
    fs::remove_file("link").unwrap();
}

