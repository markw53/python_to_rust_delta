// TODO: implementuse std::fs;
use std::os::unix::fs::MetadataExt;

pub fn extract_mode(meta: &fs::Metadata) -> Option<u32> {
    Some(meta.mode())
}

pub fn extract_mtime(meta: &fs::Metadata) -> Option<u64> {
    Some(meta.mtime() as u64)
}
