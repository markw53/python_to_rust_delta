"""
Snapshot model and snapshot creation.
"""

from dataclasses import dataclass
from typing import List
from .walker import walk
from .hasher import sha256_file
from .metadata import extract_metadata
from .symlinks import read_symlink
from .filetypes import FileType
from .utils import sort_by_path

@dataclass
class Entry:
    path: str
    type: FileType
    mode: int | None
    mtime: int | None
    size: int | None
    hash: str | None
    target: str | None

@dataclass
class Snapshot:
    root: str
    entries: List[Entry]

def create_snapshot(root: str) -> Snapshot:
    entries = []

    for w in walk(root):
        if w.type == FileType.FILE:
            meta = extract_metadata(w.abs_path)
            entries.append(
                Entry(
                    path=w.path,
                    type=w.type,
                    mode=meta.mode,
                    mtime=meta.mtime,
                    size=meta.size,
                    hash=sha256_file(w.abs_path),
                    target=None,
                )
            )
        elif w.type == FileType.DIR:
            meta = extract_metadata(w.abs_path)
            entries.append(
                Entry(
                    path=w.path,
                    type=w.type,
                    mode=meta.mode,
                    mtime=meta.mtime,
                    size=None,
                    hash=None,
                    target=None,
                )
            )
        else:  # symlink
            entries.append(
                Entry(
                    path=w.path,
                    type=w.type,
                    mode=None,
                    mtime=None,
                    size=None,
                    hash=None,
                    target=read_symlink(w.abs_path),
                )
            )

    entries = sort_by_path(entries)
    return Snapshot(root=root, entries=entries)
