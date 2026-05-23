from ..core.snapshot import Entry
from ..core.filetypes import FileType

def is_file(e: Entry) -> bool:
    return e.type == FileType.FILE

def is_dir(e: Entry) -> bool:
    return e.type == FileType.DIR

def is_symlink(e: Entry) -> bool:
    return e.type == FileType.SYMLINK
