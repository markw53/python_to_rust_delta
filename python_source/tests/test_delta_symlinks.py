from python_source.src.core.snapshot import create_snapshot
from python_source.src.delta.compute import compute_delta

def test_symlink_create(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    target = dst / "real.txt"
    target.write_text("x")

    (dst / "link").symlink_to(target)

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "symlink"
    assert ops[0].path == "link"
    assert ops[0].target == str(target)

def test_symlink_delete(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    target = src / "real.txt"
    target.write_text("x")

    (src / "link").symlink_to(target)

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "delete_file"
    assert ops[0].path == "link"

def test_symlink_target_change(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    t1 = src / "a.txt"
    t2 = dst / "b.txt"
    t1.write_text("x")
    t2.write_text("y")

    (src / "link").symlink_to(t1)
    (dst / "link").symlink_to(t2)

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "symlink"
    assert ops[0].target == str(t2)
