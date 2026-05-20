use crate::filetypes::FileType;
use crate::hasher::sha256_file;
use crate::metadata::{extract_mode, extract_mtime};
use crate::paths::normalize_path;
use crate::symlinks::read_symlink;
use crate::walker::walk;

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEntry {
    pub path: String,
    pub file_type: FileType,
    pub hash: Option<String>,
    pub mode: Option<u32>,
    pub mtime: Option<u64>,
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub entries: Vec<SnapshotEntry>,
}

pub fn create_snapshot(root: &str) -> Snapshot {
    let mut entries = Vec::new();

    let paths = walk(root);

    for full_path in paths {
        // Skip the root directory itself
        if full_path == PathBuf::from(root) {
            continue;
        }

        let rel = full_path.strip_prefix(root).unwrap();
        let rel_str = normalize_path(rel);

        let meta = match fs::symlink_metadata(&full_path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let file_type = if meta.is_file() {
            FileType::File
        } else if meta.is_dir() {
            FileType::Directory
        } else if meta.file_type().is_symlink() {
            FileType::Symlink
        } else {
            continue;
        };

        let hash = if file_type == FileType::File {
            sha256_file(full_path.to_string_lossy().as_ref()).ok()
        } else {
            None
        };

        let target = if file_type == FileType::Symlink {
            read_symlink(&full_path)
        } else {
            None
        };

        let mode = extract_mode(&meta);
        let mtime = extract_mtime(&meta);

        entries.push(SnapshotEntry {
            path: rel_str,
            file_type,
            hash,
            mode,
            mtime,
            target,
        });
    }

    // Deterministic ordering
    entries.sort_by(|a, b| a.path.cmp(&b.path));

    Snapshot { entries }
}
