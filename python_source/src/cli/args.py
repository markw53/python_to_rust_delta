"""
Argument parsing for the delta tool.
"""

import argparse

def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(description="Directory delta generator")

    sub = p.add_subparsers(dest="command", required=True)

    # compute
    c = sub.add_parser("compute", help="Compute a delta between two directories")
    c.add_argument("--src", required=True, help="Source directory")
    c.add_argument("--dst", required=True, help="Destination directory")
    c.add_argument("--out", required=True, help="Output JSON patch file")

    # apply
    a = sub.add_parser("apply", help="Apply a patch to a directory")
    a.add_argument("--root", required=True, help="Root directory to modify")
    a.add_argument("--patch", required=True, help="Patch JSON file to apply")
    a.add_argument("--dry-run", action="store_true", help="Print operations without applying")

    return p
