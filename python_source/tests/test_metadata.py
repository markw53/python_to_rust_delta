import os
from python_source.src.core.metadata import extract_metadata

def test_extract_metadata_file(tmp_path):
    p = tmp_path / "a.txt"
    p.write_text("hello")

    meta = extract_metadata(str(p))
    assert meta.size == 5
    assert meta.mode is not None
    assert meta.mtime is not None

def test_extract_metadata_dir(tmp_path):
    d = tmp_path / "folder"
    d.mkdir()

    meta = extract_metadata(str(d))
    assert meta.size is None
    assert meta.mode is not None
