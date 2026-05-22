use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/// A trait allowing extract_mode/extract_mtime to accept:
/// - &str
/// - &Path
/// - &Metadata
pub trait IntoMetadata<'a> {
    fn as_metadata(&self) -> Option<&'a fs::Metadata>;
    fn as_path(&self) -> Option<&'a Path>;
}

impl<'a> IntoMetadata<'a> for &'a fs::Metadata {
    fn as_metadata(&self) -> Option<&'a fs::Metadata> {
        Some(*self)
    }
    fn as_path(&self) -> Option<&'a Path> {
        None
    }
}

impl<'a> IntoMetadata<'a> for &'a Path {
    fn as_metadata(&self) -> Option<&'a fs::Metadata> {
        None
    }
    fn as_path(&self) -> Option<&'a Path> {
        Some(*self)
    }
}

impl<'a> IntoMetadata<'a> for &'a str {
    fn as_metadata(&self) -> Option<&'a fs::Metadata> {
        None
    }
    fn as_path(&self) -> Option<&'a Path> {
        Some(Path::new(*self))
    }
}

pub fn extract_mode<'a, T: IntoMetadata<'a>>(src: T) -> Option<u32> {
    if let Some(m) = src.as_metadata() {
        return Some(m.mode());
    }
    if let Some(p) = src.as_path() {
        let md = fs::symlink_metadata(p).ok()?;
        return Some(md.mode());
    }
    None
}

pub fn extract_mtime<'a, T: IntoMetadata<'a>>(src: T) -> Option<u64> {
    if let Some(m) = src.as_metadata() {
        return Some(m.mtime() as u64);
    }
    if let Some(p) = src.as_path() {
        let md = fs::symlink_metadata(p).ok()?;
        return Some(md.mtime() as u64);
    }
    None
}
