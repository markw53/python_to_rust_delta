import os
from python_source.src.core.snapshot import create_snapshot
from python_source.src.delta.compute import compute_delta
from python_source.src.apply.executor import apply_patch

def test_file_dir_conflict(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "x").write_text("hello")
    (dst / "x").mkdir()

    ops = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    apply_patch(str(src), ops)

    assert (src / "x").is_dir()

def test_dir_file_conflict(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "x").mkdir()
    (dst / "x").write_text("hello")

    ops = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    apply_patch(str(src), ops)

    assert (src / "x").is_file()

def test_symlink_loop(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    # Create a loop: a -> b, b -> a
    (dst / "a").symlink_to("b")
    (dst / "b").symlink_to("a")

    ops = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    apply_patch(str(src), ops)

    assert (src / "a").is_symlink()
    assert (src / "b").is_symlink()

def test_large_tree(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    # Create a deep tree
    base = dst
    for i in range(20):
        base = base / f"d{i}"
        base.mkdir()
        (base / f"f{i}.txt").write_text(str(i))

    ops = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    apply_patch(str(src), ops)

    # Verify deepest file exists
    assert (src / "d0/d1/d2/d3/d4/d5/d6/d7/d8/d9/d10/d11/d12/d13/d14/d15/d16/d17/d18/d19/f19.txt").exists()

def test_patch_stability(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (dst / "a.txt").write_text("hello")
    (dst / "b.txt").write_text("world")

    ops1 = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    ops2 = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))

    # Must be identical every time
    assert [o.op for o in ops1] == [o.op for o in ops2]
    assert [o.path for o in ops1] == [o.path for o in ops2]
