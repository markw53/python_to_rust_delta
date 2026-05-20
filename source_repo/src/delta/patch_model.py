from dataclasses import dataclass
from typing import Optional

@dataclass
class PatchOp:
    op: str
    path: Optional[str] = None
    mode: Optional[int] = None
    mtime: Optional[int] = None
    hash: Optional[str] = None
    target: Optional[str] = None
