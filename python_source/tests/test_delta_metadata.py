import os
import time
from python_source.src.core.snapshot import create_snapshot
from python_source.src.delta.compute import compute_delta

def test_chmod_change(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    p1 = src / "a.txt"
    p2 = dst / "a.txt"
    p1.write_text("x")
    p2.write_text("x")

    os.chmod(p1, 0o644)
    os.chmod(p2, 0o600)

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "chmod"
    assert ops[0].path == "a.txt"
    assert ops[0].mode == 0o600

def test_utimes_change(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    p1 = src / "a.txt"
    p2 = dst / "a.txt"
    p1.write_text("x")
    p2.write_text("x")

    # Force different mtimes
    os.utime(p1, (100000, 100000))
    os.utime(p2, (200000, 200000))

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "utimes"
    assert ops[0].mtime == 200000
