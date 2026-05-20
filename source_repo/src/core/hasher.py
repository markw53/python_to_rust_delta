"""
File hashing utilities.
"""

import hashlib
from .constants import HASH_BLOCK_SIZE

def sha256_file(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while True:
            chunk = f.read(HASH_BLOCK_SIZE)
            if not chunk:
                break
            h.update(chunk)
    return h.hexdigest()
