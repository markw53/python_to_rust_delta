pub mod apply;
pub mod delta;
pub mod filetypes;
pub mod hasher;
pub mod metadata;
pub mod patchop;
pub mod paths;
pub mod snapshot;
pub mod symlinks;
pub mod walker;

pub use apply::apply_patch;
pub use delta::compute_delta;
pub use patchop::PatchOp;
pub use snapshot::create_snapshot;
