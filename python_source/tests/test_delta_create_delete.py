import json
from python_source.src.core.snapshot import create_snapshot
from python_source.src.delta.compute import compute_delta

def test_create_file(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (dst / "a.txt").write_text("hello")

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "create_file"
    assert ops[0].path == "a.txt"

def test_delete_file(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "a.txt").write_text("hello")

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert len(ops) == 1
    assert ops[0].op == "delete_file"
    assert ops[0].path == "a.txt"

def test_create_dir(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (dst / "d").mkdir()

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert ops == [
        # create_dir must come before any nested creates
        # but here it's just one
        ops[0]
    ]
    assert ops[0].op == "create_dir"
    assert ops[0].path == "d"

def test_delete_dir(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "d").mkdir()

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    assert ops[0].op == "delete_dir"
    assert ops[0].path == "d"

def test_nested_create_ordering(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (dst / "a").mkdir()
    (dst / "a" / "b").mkdir()
    (dst / "a" / "b" / "c.txt").write_text("x")

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    # Must create in depth order: a → a/b → a/b/c.txt
    assert [o.path for o in ops] == ["a", "a/b", "a/b/c.txt"]
    assert ops[0].op == "create_dir"
    assert ops[1].op == "create_dir"
    assert ops[2].op == "create_file"

def test_nested_delete_ordering(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    (src / "a").mkdir()
    (src / "a" / "b").mkdir()
    (src / "a" / "b" / "c.txt").write_text("x")

    ops = compute_delta(
        create_snapshot(str(src)),
        create_snapshot(str(dst)),
    )

    # Must delete deepest first: a/b/c.txt → a/b → a
    assert [o.path for o in ops] == ["a/b/c.txt", "a/b", "a"]
    assert ops[0].op == "delete_file"
    assert ops[1].op == "delete_dir"
    assert ops[2].op == "delete_dir"
