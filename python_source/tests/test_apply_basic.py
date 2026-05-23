import os
from python_source.src.apply.executor import apply_patch
from python_source.src.delta.patch_model import PatchOp

def test_apply_create_file(tmp_path):
    root = tmp_path
    ops = [PatchOp(op="create_file", path="a.txt")]

    apply_patch(str(root), ops)

    assert (root / "a.txt").exists()
    assert (root / "a.txt").is_file()

def test_apply_delete_file(tmp_path):
    root = tmp_path
    p = root / "a.txt"
    p.write_text("x")

    ops = [PatchOp(op="delete_file", path="a.txt")]
    apply_patch(str(root), ops)

    assert not p.exists()

def test_apply_create_dir(tmp_path):
    root = tmp_path
    ops = [PatchOp(op="create_dir", path="d")]

    apply_patch(str(root), ops)

    assert (root / "d").is_dir()

def test_apply_delete_dir(tmp_path):
    root = tmp_path
    d = root / "d"
    d.mkdir()

    ops = [PatchOp(op="delete_dir", path="d")]
    apply_patch(str(root), ops)

    assert not d.exists()

def test_apply_modify_file(tmp_path):
    root = tmp_path
    p = root / "a.txt"
    p.write_text("hello")

    ops = [PatchOp(op="modify_file", path="a.txt")]
    apply_patch(str(root), ops)

    # modify_file truncates the file
    assert p.read_text() == ""

def test_apply_symlink(tmp_path):
    root = tmp_path
    target = root / "real.txt"
    target.write_text("x")

    ops = [PatchOp(op="symlink", path="link", target=str(target))]
    apply_patch(str(root), ops)

    link = root / "link"
    assert link.is_symlink()
    assert os.readlink(link) == str(target)

def test_apply_idempotent_create_dir(tmp_path):
    root = tmp_path
    d = root / "d"
    d.mkdir()

    ops = [PatchOp(op="create_dir", path="d")]
    apply_patch(str(root), ops)

    assert d.exists()
    assert d.is_dir()

def test_apply_idempotent_delete_file(tmp_path):
    root = tmp_path
    # file does not exist
    ops = [PatchOp(op="delete_file", path="ghost.txt")]

    apply_patch(str(root), ops)

    # still does not exist, but no error
    assert not (root / "ghost.txt").exists()
