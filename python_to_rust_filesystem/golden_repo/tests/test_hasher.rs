use filesystem_delta::hasher::sha256_file;
use std::fs;

#[test]
fn test_sha256_file() {
    let path = "testfile.txt";
    fs::write(path, "hello").unwrap();

    let hash = sha256_file(path).unwrap();
    assert_eq!(
        hash,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );

    fs::remove_file(path).unwrap();
}

