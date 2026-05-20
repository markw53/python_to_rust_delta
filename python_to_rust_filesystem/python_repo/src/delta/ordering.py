from ..core.paths import normalize_path

def depth(path: str) -> int:
    if not path:
        return 0
    return path.count("/")

def sort_deletes(paths):
    return sorted(paths, key=lambda p: (-depth(p), p))

def sort_creates(paths):
    return sorted(paths, key=lambda p: (depth(p), p))

def sort_meta(ops):
    return sorted(ops, key=lambda o: (o.op, o.path or ""))
