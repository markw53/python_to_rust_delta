"""
Enum for file types used in Entry dataclass.
"""

from enum import Enum

class FileType(Enum):
    FILE = "file"
    DIR = "dir"
    SYMLINK = "symlink"

    @staticmethod
    def from_string(s: str) -> "FileType":
        for ft in FileType:
            if ft.value == s:
                return ft
        raise ValueError(f"Unknown file type: {s}")
