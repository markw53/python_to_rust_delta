use crate::filetypes::FileType;
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEntry {
    pub path: String,
    pub file_type: FileType,
    pub contents: Option<Vec<u8>>,
    pub target: Option<String>,
    pub mode: Option<u32>,
    pub mtime: Option<u64>,
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub entries: Vec<SnapshotEntry>,
}

pub fn create_snapshot(root: &str) -> Snapshot {
    let mut entries = Vec::new();
    let root_path = PathBuf::from(root);

    for entry in walk(&root_path) {
        let rel = entry
            .strip_prefix(&root_path)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let md = fs::symlink_metadata(&entry).unwrap();
        let mode = Some(md.mode());
        let mtime = Some(md.mtime() as u64);

        if md.file_type().is_symlink() {
            let target = fs::read_link(&entry).unwrap().to_string_lossy().to_string();
            entries.push(SnapshotEntry {
                path: rel,
                file_type: FileType::Symlink,
                contents: None,
                target: Some(target),
                mode,
                mtime,
                hash: None,
            });
        } else if md.is_dir() {
            entries.push(SnapshotEntry {
                path: rel,
                file_type: FileType::Directory,
                contents: None,
                target: None,
                mode,
                mtime,
                hash: None,
            });
        } else {
            let contents = fs::read(&entry).unwrap();
            let hash = Some(crate::hasher::sha256_file(entry.to_string_lossy().as_ref()).unwrap());

            entries.push(SnapshotEntry {
                path: rel,
                file_type: FileType::File,
                contents: Some(contents),
                target: None,
                mode,
                mtime,
                hash,
            });
        }
    }

    Snapshot { entries }
}

fn walk(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if !root.exists() {
        return out;
    }

    let mut stack = vec![root.to_path_buf()];

    while let Some(path) = stack.pop() {
        out.push(path.clone());

        if path.is_dir() {
            if let Ok(read) = fs::read_dir(&path) {
                for entry in read.flatten() {
                    stack.push(entry.path());
                }
            }
        }
    }

    out.sort();
    out
}
