"""
Constants used across the delta generator system.
"""

HASH_BLOCK_SIZE = 8192

FILE_TYPE_FILE = "file"
FILE_TYPE_DIR = "dir"
FILE_TYPE_SYMLINK = "symlink"

SUPPORTED_FILE_TYPES = {
    FILE_TYPE_FILE,
    FILE_TYPE_DIR,
    FILE_TYPE_SYMLINK,
}
