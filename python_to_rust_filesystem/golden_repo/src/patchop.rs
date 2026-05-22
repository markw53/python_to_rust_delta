use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOp {
    pub op: String,
    pub path: String,
    pub target: Option<String>,
    pub contents: Option<Vec<u8>>,
    pub mode: Option<u32>,
    pub mtime: Option<u64>,
}
