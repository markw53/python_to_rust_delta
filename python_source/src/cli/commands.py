"""
Command handlers for the delta tool.
"""

import json
from ..core.snapshot import create_snapshot
from ..delta.compute import compute_delta
from ..delta.patch_serialize import to_dict_list
from ..apply.executor import apply_patch
from ..delta.patch_model import PatchOp

def cmd_compute(src: str, dst: str, out: str) -> None:
    snap_src = create_snapshot(src)
    snap_dst = create_snapshot(dst)

    ops = compute_delta(snap_src, snap_dst)
    data = to_dict_list(ops)

    with open(out, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)


def cmd_apply(root: str, patch_file: str, dry_run: bool) -> None:
    with open(patch_file, "r", encoding="utf-8") as f:
        raw = json.load(f)

    ops = [PatchOp(**d) for d in raw]
    apply_patch(root, ops, dry_run=dry_run)
