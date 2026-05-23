import os
from python_source.src.core.paths import normalize_path

def test_normalize_simple(tmp_path):
    root = tmp_path
    p = root / "a" / "b.txt"
    p.parent.mkdir()
    p.write_text("x")

    assert normalize_path(str(root), str(p)) == "a/b.txt"

def test_normalize_root(tmp_path):
    assert normalize_path(str(tmp_path), str(tmp_path)) == ""

def test_normalize_backslashes(tmp_path):
    root = tmp_path
    p = root / "a" / "b.txt"
    p.parent.mkdir()
    p.write_text("x")

    # simulate Windows-style path
    win_path = str(p).replace("/", "\\")
    assert normalize_path(str(root), win_path) == "a/b.txt"
