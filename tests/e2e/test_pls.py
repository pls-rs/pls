from tests.e2e.utils import run_pls


def test_pls():
    proc = run_pls()
    assert proc.returncode == 0
