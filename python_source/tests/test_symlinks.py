from python_source.src.core.symlinks import read_symlink

def test_read_symlink(tmp_path):
    target = tmp_path / "real.txt"
    target.write_text("x")

    link = tmp_path / "link.txt"
    link.symlink_to(target)

    assert read_symlink(str(link)) == str(target)
