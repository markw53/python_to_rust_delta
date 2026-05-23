from typing import List
from .patch_model import PatchOp

def validate_patch(ops: List[PatchOp]) -> None:
    # keep this intentionally light but realistic
    for op in ops:
        if not op.op:
            raise ValueError("Patch operation missing 'op'")
        if op.op in {"create_file", "create_dir", "delete_file", "delete_dir",
                     "modify_file", "chmod", "utimes", "symlink"}:
            if op.path is None:
                raise ValueError(f"Patch op {op.op} requires 'path'")
