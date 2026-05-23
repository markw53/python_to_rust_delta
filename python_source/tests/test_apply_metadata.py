import os
from python_source.src.apply.executor import apply_patch
from python_source.src.delta.patch_model import PatchOp

def test_apply_chmod(tmp_path):
    root = tmp_path
    p = root / "a.txt"
    p.write_text("x")

    ops = [PatchOp(op="chmod", path="a.txt", mode=0o600)]
    apply_patch(str(root), ops)

    st = os.stat(p)
    assert (st.st_mode & 0o777) == 0o600

def test_apply_utimes(tmp_path):
    root = tmp_path
    p = root / "a.txt"
    p.write_text("x")

    ops = [PatchOp(op="utimes", path="a.txt", mtime=123456)]
    apply_patch(str(root), ops)

    st = os.stat(p)
    assert int(st.st_mtime) == 123456

def test_apply_chmod_no_mode(tmp_path):
    root = tmp_path
    p = root / "a.txt"
    p.write_text("x")

    ops = [PatchOp(op="chmod", path="a.txt", mode=None)]
    apply_patch(str(root), ops)

    # Should not crash, mode unchanged
    st = os.stat(p)
    assert (st.st_mode & 0o777) != 0  # still valid

def test_apply_utimes_no_mtime(tmp_path):
    root = tmp_path
    p = root / "a.txt"
    p.write_text("x")

    ops = [PatchOp(op="utimes", path="a.txt", mtime=None)]
    apply_patch(str(root), ops)

    # Should not crash, mtime unchanged
    st = os.stat(p)
    assert st.st_mtime > 0
