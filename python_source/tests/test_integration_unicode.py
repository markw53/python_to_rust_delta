import os
from python_source.src.core.snapshot import create_snapshot
from python_source.src.delta.compute import compute_delta
from python_source.src.apply.executor import apply_patch

def test_unicode_paths(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    filename = "файл.txt"
    (dst / filename).write_text("hello")

    ops = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    apply_patch(str(src), ops)

    assert (src / filename).exists()

def test_unicode_nested(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()

    d = dst / "данные"
    d.mkdir()
    (d / "тест.txt").write_text("x")

    ops = compute_delta(create_snapshot(str(src)), create_snapshot(str(dst)))
    apply_patch(str(src), ops)

    assert (src / "данные" / "тест.txt").exists()
