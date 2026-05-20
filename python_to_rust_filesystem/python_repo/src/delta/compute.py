from typing import List, Dict
from ..core.snapshot import Snapshot, Entry
from ..core.filetypes import FileType
from .patch_model import PatchOp
from .ordering import sort_deletes, sort_creates, sort_meta
from .match import build_maps
from .classify import is_file, is_dir, is_symlink
from .patch_validate import validate_patch

def _same_type(a: Entry, b: Entry) -> bool:
    return a.type == b.type

def _add_delete_ops(path: str, e: Entry, out: List[PatchOp]) -> None:
    if is_file(e) or is_symlink(e):
        out.append(PatchOp(op="delete_file", path=path))
    elif is_dir(e):
        out.append(PatchOp(op="delete_dir", path=path))

def _add_create_ops(path: str, e: Entry, out: List[PatchOp]) -> None:
    if is_dir(e):
        out.append(PatchOp(op="create_dir", path=path))
    elif is_file(e):
        out.append(PatchOp(op="create_file", path=path, hash=e.hash))
    elif is_symlink(e):
        out.append(PatchOp(op="symlink", path=path, target=e.target))

def compute_delta(src: Snapshot, dst: Snapshot) -> List[PatchOp]:
    src_map: Dict[str, Entry] = build_maps(src.entries)
    dst_map: Dict[str, Entry] = build_maps(dst.entries)

    src_only = [p for p in src_map.keys() if p not in dst_map]
    dst_only = [p for p in dst_map.keys() if p not in src_map]

    ops: List[PatchOp] = []
    meta: List[PatchOp] = []

    # same-path entries
    for path, se in src_map.items():
        if path not in dst_map:
            continue
        de = dst_map[path]

        if not _same_type(se, de):
            _add_delete_ops(path, se, ops)
            _add_create_ops(path, de, ops)
            continue

        if is_file(se):
            if se.hash != de.hash:
                meta.append(PatchOp(op="modify_file", path=path))
            else:
                if se.mode != de.mode:
                    meta.append(PatchOp(op="chmod", path=path, mode=de.mode))
                if se.mtime != de.mtime:
                    meta.append(PatchOp(op="utimes", path=path, mtime=de.mtime))
        elif is_dir(se):
            if se.mode != de.mode:
                meta.append(PatchOp(op="chmod", path=path, mode=de.mode))
        elif is_symlink(se):
            if se.target != de.target:
                meta.append(PatchOp(op="symlink", path=path, target=de.target))

    # deletes
    for path in sort_deletes(src_only):
        e = src_map[path]
        _add_delete_ops(path, e, ops)

    # creates
    for path in sort_creates(dst_only):
        e = dst_map[path]
        _add_create_ops(path, e, ops)

    # metadata
    meta = sort_meta(meta)
    ops.extend(meta)

    validate_patch(ops)
    return ops
