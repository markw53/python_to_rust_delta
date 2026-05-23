"""
Metadata extraction for files and directories.
"""

import os
from dataclasses import dataclass
from .errors import MetadataError

@dataclass
class FileMetadata:
    mode: int
    mtime: int
    size: int | None = None

def extract_metadata(path: str) -> FileMetadata:
    try:
        st = os.lstat(path)
        return FileMetadata(
            mode=st.st_mode,
            mtime=int(st.st_mtime),
            size=st.st_size if not os.path.isdir(path) else None,
        )
    except Exception as e:
        raise MetadataError(str(e))
