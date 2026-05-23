"""
Filesystem walker that yields normalized paths and file types.
"""

import os
from dataclasses import dataclass
from .paths import normalize_path
from .filetypes import FileType
from .errors import SnapshotError

@dataclass
class WalkEntry:
    path: str
    type: FileType
    abs_path: str

def walk(root: str):
    try:
        for dirpath, dirs, files in os.walk(root):
            # directories
            for d in dirs:
                p = os.path.join(dirpath, d)
                yield WalkEntry(
                    path=normalize_path(root, p),
                    type=FileType.DIR,
                    abs_path=p,
                )

            # files
            for f in files:
                p = os.path.join(dirpath, f)
                if os.path.islink(p):
                    continue
                yield WalkEntry(
                    path=normalize_path(root, p),
                    type=FileType.FILE,
                    abs_path=p,
                )

            # symlinks
            for name in os.listdir(dirpath):
                p = os.path.join(dirpath, name)
                if os.path.islink(p):
                    yield WalkEntry(
                        path=normalize_path(root, p),
                        type=FileType.SYMLINK,
                        abs_path=p,
                    )
    except Exception as e:
        raise SnapshotError(str(e))
