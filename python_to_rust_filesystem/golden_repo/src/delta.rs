use crate::snapshot::{Snapshot, SnapshotEntry};
use crate::filetypes::FileType;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOp {
    pub op: String,
    pub path: String,
    pub hash: Option<String>,
    pub mode: Option<u32>,
    pub mtime: Option<u64>,
    pub target: Option<String>,
}

fn index_snapshot(snap: &Snapshot) -> BTreeMap<String, &SnapshotEntry> {
    let mut map = BTreeMap::new();
    for e in &snap.entries {
        map.insert(e.path.clone(), e);
    }
    map
}

pub fn compute_delta(src: Snapshot, dst: Snapshot) -> Vec<PatchOp> {
    let src_map = index_snapshot(&src);
    let dst_map = index_snapshot(&dst);

    let mut ops: Vec<PatchOp> = Vec::new();

    // Deletions and modifications
    for (path, src_e) in &src_map {
        match dst_map.get(path) {
            None => {
                // delete
                match src_e.file_type {
                    FileType::File | FileType::Symlink => ops.push(PatchOp {
                        op: "delete_file".to_string(),
                        path: path.clone(),
                        hash: None,
                        mode: None,
                        mtime: None,
                        target: None,
                    }),
                    FileType::Directory => ops.push(PatchOp {
                        op: "delete_dir".to_string(),
                        path: path.clone(),
                        hash: None,
                        mode: None,
                        mtime: None,
                        target: None,
                    }),
                }
            }
            Some(dst_e) => {
                // type change
                if src_e.file_type != dst_e.file_type {
                    // delete old
                    match src_e.file_type {
                        FileType::File | FileType::Symlink => ops.push(PatchOp {
                            op: "delete_file".to_string(),
                            path: path.clone(),
                            hash: None,
                            mode: None,
                            mtime: None,
                            target: None,
                        }),
                        FileType::Directory => ops.push(PatchOp {
                            op: "delete_dir".to_string(),
                            path: path.clone(),
                            hash: None,
                            mode: None,
                            mtime: None,
                            target: None,
                        }),
                    }
                    // create new
                    match dst_e.file_type {
                        FileType::File => ops.push(PatchOp {
                            op: "create_file".to_string(),
                            path: path.clone(),
                            hash: dst_e.hash.clone(),
                            mode: None,
                            mtime: None,
                            target: None,
                        }),
                        FileType::Directory => ops.push(PatchOp {
                            op: "create_dir".to_string(),
                            path: path.clone(),
                            hash: None,
                            mode: None,
                            mtime: None,
                            target: None,
                        }),
                        FileType::Symlink => ops.push(PatchOp {
                            op: "symlink".to_string(),
                            path: path.clone(),
                            hash: None,
                            mode: None,
                            mtime: None,
                            target: dst_e.target.clone(),
                        }),
                    }
                } else {
                    // same type → check content/metadata
                    match src_e.file_type {
                        FileType::File => {
                            if src_e.hash != dst_e.hash {
                                ops.push(PatchOp {
                                    op: "modify_file".to_string(),
                                    path: path.clone(),
                                    hash: dst_e.hash.clone(),
                                    mode: None,
                                    mtime: None,
                                    target: None,
                                });
                            }
                        }
                        FileType::Symlink => {
                            if src_e.target != dst_e.target {
                                ops.push(PatchOp {
                                    op: "symlink".to_string(),
                                    path: path.clone(),
                                    hash: None,
                                    mode: None,
                                    mtime: None,
                                    target: dst_e.target.clone(),
                                });
                            }
                        }
                        FileType::Directory => {}
                    }

                    // chmod
                    if src_e.mode != dst_e.mode {
                        if let Some(mode) = dst_e.mode {
                            ops.push(PatchOp {
                                op: "chmod".to_string(),
                                path: path.clone(),
                                hash: None,
                                mode: Some(mode),
                                mtime: None,
                                target: None,
                            });
                        }
                    }

                    // utimes
                    if src_e.mtime != dst_e.mtime {
                        if let Some(mtime) = dst_e.mtime {
                            ops.push(PatchOp {
                                op: "utimes".to_string(),
                                path: path.clone(),
                                hash: None,
                                mode: None,
                                mtime: Some(mtime),
                                target: None,
                            });
                        }
                    }
                }
            }
        }
    }

    // Creations
    for (path, dst_e) in &dst_map {
        if !src_map.contains_key(path) {
            match dst_e.file_type {
                FileType::File => ops.push(PatchOp {
                    op: "create_file".to_string(),
                    path: path.clone(),
                    hash: dst_e.hash.clone(),
                    mode: None,
                    mtime: None,
                    target: None,
                }),
                FileType::Directory => ops.push(PatchOp {
                    op: "create_dir".to_string(),
                    path: path.clone(),
                    hash: None,
                    mode: None,
                    mtime: None,
                    target: None,
                }),
                FileType::Symlink => ops.push(PatchOp {
                    op: "symlink".to_string(),
                    path: path.clone(),
                    hash: None,
                    mode: None,
                    mtime: None,
                    target: dst_e.target.clone(),
                }),
            }
        }
    }

    // Deterministic ordering: by path, then op
    ops.sort_by(|a, b| match a.path.cmp(&b.path) {
        std::cmp::Ordering::Equal => a.op.cmp(&b.op),
        other => other,
    });

    ops
}
