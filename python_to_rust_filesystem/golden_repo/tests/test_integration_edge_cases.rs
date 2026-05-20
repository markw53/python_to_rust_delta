use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use std::path::Path;

#[test]
fn test_file_dir_conflict() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a", "x").unwrap();
    fs::create_dir("dst/a").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    apply_patch("src", ops).unwrap();

    assert!(Path::new("src/a").is_dir());

    fs::remove_dir_all("src").unwrap();
    fs::remove_dir_all("dst").unwrap();
}

#[test]
fn test_dir_file_conflict() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::create_dir("src/a").unwrap();
    fs::write("dst/a", "x").unwrap();

    let ops = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    apply_patch("src", ops).unwrap();

    assert!(Path::new("src/a").is_file());

    fs::remove_dir_all("src").unwrap();
    fs::remove_dir_all("dst").unwrap();
}

#[test]
fn test_symlink_loop() {
    fs::create_dir("d").unwrap();
    std::os::unix::fs::symlink("loop", "d/loop").unwrap();

    let snap = create_snapshot("d");
    assert!(snap
        .entries
        .iter()
        .any(|e| e.file_type.to_string().contains("Symlink")));

    fs::remove_file("d/loop").unwrap();
    fs::remove_dir("d").unwrap();
}

#[test]
fn test_large_tree() {
    fs::create_dir("d").unwrap();
    for i in 0..200 {
        fs::write(format!("d/f{i}.txt"), "x").unwrap();
    }

    let snap = create_snapshot("d");
    assert_eq!(snap.entries.len(), 200);

    fs::remove_dir_all("d").unwrap();
}

#[test]
fn test_patch_stability() {
    fs::create_dir("src").unwrap();
    fs::create_dir("dst").unwrap();

    fs::write("src/a.txt", "1").unwrap();
    fs::write("dst/a.txt", "2").unwrap();

    let ops1 = compute_delta(create_snapshot("src"), create_snapshot("dst"));
    let ops2 = compute_delta(create_snapshot("src"), create_snapshot("dst"));

    assert_eq!(
        serde_json::to_string(&ops1).unwrap(),
        serde_json::to_string(&ops2).unwrap()
    );

    fs::remove_dir_all("src").unwrap();
    fs::remove_dir_all("dst").unwrap();
}
