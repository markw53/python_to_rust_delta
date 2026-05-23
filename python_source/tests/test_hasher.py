from python_source.src.core.hasher import sha256_file

def test_sha256_file(tmp_path):
    p = tmp_path / "a.txt"
    p.write_text("hello")

    h = sha256_file(str(p))
    assert len(h) == 64
    assert h != sha256_file(str(p))  # content same → hash same
