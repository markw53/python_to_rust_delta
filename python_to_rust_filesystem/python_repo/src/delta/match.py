from typing import Dict
from ..core.snapshot import Entry

def build_maps(entries):
    by_path: Dict[str, Entry] = {}
    for e in entries:
        by_path[e.path] = e
    return by_path
