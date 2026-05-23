from python_source.src.core.snapshot import create_snapshot
from python_source.src.delta.compute import compute_delta

def test_modify_file_hash_change(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "a.txt").write_text("hello")
    (dst / "a.txt").write_text("world")

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "modify_file"
    assert ops[0].path == "a.txt"

def test_modify_file_no_change(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "a.txt").write_text("hello")
    (dst / "a.txt").write_text("hello")

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert ops == []

def test_type_change_file_to_dir(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "x").write_text("hello")
    (dst / "x").mkdir()

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert ops[0].op == "delete_file"
    assert ops[1].op == "create_dir"
    assert ops[0].path == "x"
    assert ops[1].path == "x"

def test_type_change_dir_to_file(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "x").mkdir()
    (dst / "x").write_text("hello")

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert ops[0].op == "delete_dir"
    assert ops[1].op == "create_file"
