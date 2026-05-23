from python_source.src.core.snapshot import create_snapshot
from python_source.src.core.filetypes import FileType

def test_snapshot_simple(tmp_path):
    (tmp_path / "a").mkdir()
    (tmp_path / "a" / "b.txt").write_text("hello")
    (tmp_path / "c.txt").write_text("world")

    snap = create_snapshot(str(tmp_path))
    paths = [e.path for e in snap.entries]

    assert paths == ["a", "a/b.txt", "c.txt"]

def test_snapshot_symlink(tmp_path):
    target = tmp_path / "real.txt"
    target.write_text("x")

    link = tmp_path / "link.txt"
    link.symlink_to(target)

    snap = create_snapshot(str(tmp_path))
    entry = next(e for e in snap.entries if e.path == "link.txt")

    assert entry.type == FileType.SYMLINK
    assert entry.target == str(target)
