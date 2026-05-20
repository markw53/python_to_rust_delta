use crate::delta::PatchOp;
use anyhow::{Context, Result};
use filetime::FileTime;
use std::fs;
use std::io::Write;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};

pub fn apply_patch(root: &str, ops: Vec<PatchOp>) -> Result<()> {
    for op in ops {
        let full = PathBuf::from(root).join(&op.path);

        match op.op.as_str() {
            "create_dir" => {
                fs::create_dir_all(&full)
                    .with_context(|| format!("Failed to create dir {:?}", full))?;
            }

            "delete_dir" => {
                if full.exists() {
                    fs::remove_dir_all(&full)
                        .with_context(|| format!("Failed to delete dir {:?}", full))?;
                }
            }

            "create_file" => {
                if let Some(parent) = full.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut f = fs::File::create(&full)
                    .with_context(|| format!("Failed to create file {:?}", full))?;
                if let Some(hash) = &op.hash {
                    // Python version writes empty file; content is not stored in patch
                    // So we do the same: empty file, hash is informational
                    let _ = hash;
                }
                f.flush()?;
            }

            "delete_file" => {
                if full.exists() {
                    fs::remove_file(&full)
                        .with_context(|| format!("Failed to delete file {:?}", full))?;
                }
            }

            "modify_file" => {
                // Same as create_file: overwrite with empty file
                if let Some(parent) = full.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut f = fs::File::create(&full)
                    .with_context(|| format!("Failed to modify file {:?}", full))?;
                let _ = op.hash; // informational only
                f.flush()?;
            }

            "chmod" => {
                if let Some(mode) = op.mode {
                    let mut perms = fs::metadata(&full)?.permissions();
                    perms.set_mode(mode);
                    fs::set_permissions(&full, perms)?;
                }
            }

            "utimes" => {
                if let Some(mtime) = op.mtime {
                    let ft = FileTime::from_unix_time(mtime as i64, 0);
                    filetime::set_file_mtime(&full, ft)?;
                }
            }

            "symlink" => {
                if full.exists() {
                    fs::remove_file(&full).ok();
                }
                if let Some(parent) = full.parent() {
                    fs::create_dir_all(parent)?;
                }
                let target = op.target.clone().unwrap_or_default();
                symlink(&target, &full).with_context(|| {
                    format!("Failed to create symlink {:?} -> {:?}", full, target)
                })?;
            }

            other => {
                return Err(anyhow::anyhow!("Unknown op: {}", other));
            }
        }
    }

    Ok(())
}
