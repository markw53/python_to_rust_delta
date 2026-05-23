import os
from python_source.src.core.walker import walk
from python_source.src.core.filetypes import FileType

def test_walk_files_and_dirs(tmp_path):
    (tmp_path / "d").mkdir()
    (tmp_path / "d" / "x.txt").write_text("hello")
    (tmp_path / "y.txt").write_text("world")

    entries = list(walk(str(tmp_path)))
    paths = sorted(e.path for e in entries)

    assert paths == ["d", "d/x.txt", "y.txt"]

def test_walk_symlink(tmp_path):
    target = tmp_path / "real.txt"
    target.write_text("data")

    link = tmp_path / "link.txt"
    link.symlink_to(target)

    entries = list(walk(str(tmp_path)))
    types = {e.path: e.type for e in entries}

    assert types["link.txt"] == FileType.SYMLINK
