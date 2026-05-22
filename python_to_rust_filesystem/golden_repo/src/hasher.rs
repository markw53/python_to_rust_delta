use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn sha256_file(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(Path::new(path))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
