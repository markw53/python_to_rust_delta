from typing import List, Dict, Any
from .patch_model import PatchOp

def to_dict_list(ops: List[PatchOp]) -> List[Dict[str, Any]]:
    out: List[Dict[str, Any]] = []

    for op in ops:
        d: Dict[str, Any] = {"op": op.op}

        if op.path is not None:
            d["path"] = op.path
        if op.mode is not None:
            d["mode"] = op.mode
        if op.mtime is not None:
            d["mtime"] = op.mtime
        if op.hash is not None:
            d["hash"] = op.hash
        if op.target is not None:
            d["target"] = op.target

        out.append(d)

    return out
