"""
Small utility helpers.
"""

def sort_by_path(entries):
    return sorted(entries, key=lambda e: e.path)
