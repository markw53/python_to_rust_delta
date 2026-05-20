"""
Symlink operations.
"""

import os

def apply_symlink(path: str, target: str | None) -> None:
    if target is None:
        raise ValueError("symlink op requires target")

    # Remove existing file or symlink
    if os.path.lexists(path):
        os.remove(path)

    # Create new symlink
    os.symlink(target, path)
