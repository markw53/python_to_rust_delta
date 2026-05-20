use filesystem_delta::create_snapshot;
use filesystem_delta::filetypes::FileType;
use std::fs;

#[test]
fn test_snapshot_simple() {
    fs::create_dir("d").unwrap();
    fs::write("d/a.txt", "hello").unwrap();

    let snap = create_snapshot("d");

    assert_eq!(snap.entries.len(), 1);
    let e = &snap.entries[0];

    assert_eq!(e.path, "a.txt");
    assert_eq!(e.file_type, FileType::File);
    assert!(e.hash.is_some());
}

#[test]
fn test_snapshot_symlink() {
    fs::write("real.txt", "x").unwrap();
    std::os::unix::fs::symlink("real.txt", "link").unwrap();

    let snap = create_snapshot(".");

    let link = snap.entries.iter().find(|e| e.path == "link").unwrap();
    assert_eq!(link.file_type, FileType::Symlink);
    assert_eq!(link.target.as_deref(), Some("real.txt"));

    fs::remove_file("real.txt").unwrap();
    fs::remove_file("link").unwrap();
}

