"""
Metadata operations: chmod and utimes.
"""

import os
import time

def apply_chmod(path: str, mode: int | None) -> None:
    if mode is None:
        return
    try:
        os.chmod(path, mode)
    except NotImplementedError:
        # Windows: ignore silently
        pass

def apply_utimes(path: str, mtime: int | None) -> None:
    if mtime is None:
        return
    os.utime(path, (mtime, mtime))
