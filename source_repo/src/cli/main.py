"""
CLI entry point for the delta tool.
"""

from .args import build_parser
from .commands import cmd_compute, cmd_apply

def main() -> None:
    parser = build_parser()
    args = parser.parse_args()

    match args.command:
        case "compute":
            cmd_compute(args.src, args.dst, args.out)
        case "apply":
            cmd_apply(args.root, args.patch, args.dry_run)
        case _:
            raise ValueError(f"Unknown command: {args.command}")
