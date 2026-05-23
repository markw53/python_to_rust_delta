"""
File-level operations.
"""

import os

def apply_create_file(path: str) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "wb"):
        pass

def apply_delete_file(path: str) -> None:
    if os.path.exists(path) or os.path.islink(path):
        os.remove(path)

def apply_modify_file(path: str) -> None:
    # For this task, modifying a file means truncating it.
    with open(path, "wb"):
        pass
