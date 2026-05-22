use crate::patchop::PatchOp;
use std::fs;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

fn remove_if_exists(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(md) => {
            if md.file_type().is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
        Err(_) => {}
    }
    Ok(())
}

fn apply_metadata(path: &Path, mode: Option<u32>, mtime: Option<u64>) -> io::Result<()> {
    if let Some(m) = mode {
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(m);
        fs::set_permissions(path, perms)?;
    }

    if let Some(t) = mtime {
        let atime = fs::metadata(path)?.atime() as u64;
        filetime::set_file_times(
            path,
            filetime::FileTime::from_unix_time(t as i64, 0),
            filetime::FileTime::from_unix_time(atime as i64, 0),
        )?;
    }

    Ok(())
}

pub fn apply_patch(root: &str, ops: Vec<PatchOp>) -> io::Result<()> {
    let root = PathBuf::from(root);

    for op in ops {
        let mut rel = op.path.as_str();
        rel = rel.trim_start_matches('/');
        rel = rel.trim_start_matches("./");

        let full = root.join(rel);

        match op.op.as_str() {
            "create_file" => {
                remove_if_exists(&full)?;
                fs::write(&full, op.contents.unwrap_or_default())?;
                apply_metadata(&full, op.mode, op.mtime)?;
            }

            "modify_file" => {
                remove_if_exists(&full)?;
                fs::write(&full, op.contents.unwrap_or_default())?;
                apply_metadata(&full, op.mode, op.mtime)?;
            }

            "create_dir" => {
                remove_if_exists(&full)?;
                fs::create_dir_all(&full)?;
                apply_metadata(&full, op.mode, op.mtime)?;
            }

            "create_symlink" => {
                remove_if_exists(&full)?;
                std::os::unix::fs::symlink(op.target.unwrap(), &full)?;
                apply_metadata(&full, op.mode, op.mtime)?;
            }

            "delete" => {
                remove_if_exists(&full)?;
            }

            other => panic!("unknown op {}", other),
        }
    }

    Ok(())
}
