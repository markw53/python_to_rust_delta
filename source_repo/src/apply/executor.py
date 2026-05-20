"""
Patch executor: applies PatchOp operations to the filesystem.
"""

import os
from typing import List

from .file_ops import (
    apply_create_file,
    apply_delete_file,
    apply_modify_file,
)
from .dir_ops import (
    apply_create_dir,
    apply_delete_dir,
)
from .meta_ops import (
    apply_chmod,
    apply_utimes,
)
from .symlink_ops import apply_symlink

from ..delta.patch_model import PatchOp


def apply_patch(root: str, ops: List[PatchOp], dry_run: bool = False) -> None:
    for op in ops:
        if dry_run:
            print(op)
            continue

        # All ops except maybe future ones require a path
        if op.path is None:
            raise ValueError(f"Patch op '{op.op}' requires a path")

        full = os.path.join(root, op.path)

        match op.op:
            case "create_dir":
                apply_create_dir(full)
            case "delete_dir":
                apply_delete_dir(full)
            case "create_file":
                apply_create_file(full)
            case "delete_file":
                apply_delete_file(full)
            case "modify_file":
                apply_modify_file(full)
            case "chmod":
                apply_chmod(full, op.mode)
            case "utimes":
                apply_utimes(full, op.mtime)
            case "symlink":
                apply_symlink(full, op.target)
            case _:
                raise ValueError(f"Unknown patch op: {op.op}")
