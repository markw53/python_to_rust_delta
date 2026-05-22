use crate::filetypes::FileType;
use crate::patchop::PatchOp;
use crate::snapshot::{Snapshot, SnapshotEntry};

pub fn compute_delta(src: Snapshot, dst: Snapshot) -> Vec<PatchOp> {
    let mut ops = Vec::new();

    let src_map = src
        .entries
        .into_iter()
        .map(|e| (e.path.clone(), e))
        .collect::<std::collections::HashMap<_, _>>();
    let dst_map = dst
        .entries
        .into_iter()
        .map(|e| (e.path.clone(), e))
        .collect::<std::collections::HashMap<_, _>>();

    // Deletions + modifications
    for (path, s) in src_map.iter() {
        if let Some(d) = dst_map.get(path) {
            match (&s.file_type, &d.file_type) {
                (FileType::File, FileType::File) => {
                    if s.contents != d.contents {
                        ops.push(PatchOp {
                            op: "modify_file".into(),
                            path: path.clone(),
                            contents: d.contents.clone(),
                            target: None,
                            mode: d.mode,
                            mtime: d.mtime,
                        });
                    }
                }
                (FileType::Directory, FileType::Directory) => {}
                (FileType::Symlink, FileType::Symlink) => {
                    if s.target != d.target {
                        ops.push(PatchOp {
                            op: "symlink".into(),
                            path: path.clone(),
                            target: d.target.clone(),
                            contents: None,
                            mode: d.mode,
                            mtime: d.mtime,
                        });
                    }
                }
                _ => {
                    ops.push(delete_op(path));
                    ops.push(create_from_entry(d));
                }
            }
        } else {
            ops.push(delete_op(path));
        }
    }

    // Creations
    for (path, d) in dst_map.iter() {
        if !src_map.contains_key(path) {
            ops.push(create_from_entry(d));
        }
    }

    ops
}

fn delete_op(path: &str) -> PatchOp {
    PatchOp {
        op: "delete_file".into(),
        path: path.into(),
        contents: None,
        target: None,
        mode: None,
        mtime: None,
    }
}

fn create_from_entry(e: &SnapshotEntry) -> PatchOp {
    match e.file_type {
        FileType::File => PatchOp {
            op: "file".into(),
            path: e.path.clone(),
            contents: e.contents.clone(),
            target: None,
            mode: e.mode,
            mtime: e.mtime,
        },
        FileType::Directory => PatchOp {
            op: "mkdir".into(),
            path: e.path.clone(),
            contents: None,
            target: None,
            mode: e.mode,
            mtime: e.mtime,
        },
        FileType::Symlink => PatchOp {
            op: "symlink".into(),
            path: e.path.clone(),
            target: e.target.clone(),
            contents: None,
            mode: e.mode,
            mtime: e.mtime,
        },
    }
}
