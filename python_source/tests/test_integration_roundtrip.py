import os
from python_source.src.core.snapshot import create_snapshot
from python_source.src.delta.compute import compute_delta
from python_source.src.apply.executor import apply_patch

def apply_and_resnapshot(src, dst):
    ops = compute_delta(create_snapshot(src), create_snapshot(dst))
    apply_patch(src, ops)
    return create_snapshot(src)

def test_roundtrip_simple(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (dst / "a.txt").write_text("hello")

    snap_after = apply_and_resnapshot(str(src), str(dst))
    snap_dst = create_snapshot(str(dst))

    assert [e.path for e in snap_after.entries] == [e.path for e in snap_dst.entries]

def test_roundtrip_nested(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (dst / "a").mkdir()
    (dst / "a" / "b").mkdir()
    (dst / "a" / "b" / "c.txt").write_text("x")

    snap_after = apply_and_resnapshot(str(src), str(dst))
    snap_dst = create_snapshot(str(dst))

    assert [e.path for e in snap_after.entries] == [e.path for e in snap_dst.entries]

def test_roundtrip_modify(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "a.txt").write_text("hello")
    (dst / "a.txt").write_text("world")

    snap_after = apply_and_resnapshot(str(src), str(dst))
    snap_dst = create_snapshot(str(dst))

    assert [e.hash for e in snap_after.entries] == [e.hash for e in snap_dst.entries]

def test_roundtrip_symlink(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    t = dst / "real.txt"
    t.write_text("x")
    (dst / "link").symlink_to(t)

    snap_after = apply_and_resnapshot(str(src), str(dst))
    snap_dst = create_snapshot(str(dst))

    assert [e.target for e in snap_after.entries] == [e.target for e in snap_dst.entries]

def test_roundtrip_idempotent(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (dst / "a.txt").write_text("hello")

    # First apply
    apply_and_resnapshot(str(src), str(dst))

    # Second apply (should do nothing)
    ops = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    assert ops == []
