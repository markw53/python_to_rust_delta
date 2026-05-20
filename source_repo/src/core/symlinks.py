"""
Symlink utilities.
"""

import os

def read_symlink(path: str) -> str:
    return os.readlink(path)
