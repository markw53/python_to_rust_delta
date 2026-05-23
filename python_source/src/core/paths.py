"""
Path normalization utilities.
"""

import os
from .errors import InvalidPath

def normalize_path(root: str, path: str) -> str:
    """
    Normalize a filesystem path relative to a root directory.
    Ensures forward slashes and no leading './'.
    """
    try:
        rel = os.path.relpath(path, root)
    except Exception as e:
        raise InvalidPath(str(e))

    rel = rel.replace("\\", "/")
    if rel == ".":
        return ""
    return rel
