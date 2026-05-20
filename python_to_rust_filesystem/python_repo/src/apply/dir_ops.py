"""
Directory-level operations.
"""

import os
import shutil

def apply_create_dir(path: str) -> None:
    os.makedirs(path, exist_ok=True)

def apply_delete_dir(path: str) -> None:
    if os.path.isdir(path):
        shutil.rmtree(path)
