import re

import pytest

from tests.e2e.utils import run_pls


@pytest.mark.parametrize(
    "arg",
    ["-v", "--version"],
)
def test_version(arg: str):
    proc = run_pls([arg])
    assert re.match(r"pls \d+\.\d+\.\d+", proc.stdout)


@pytest.mark.parametrize(
    "arg",
    ["-h", "--help"],
)
def test_help(arg: str):
    proc = run_pls([arg])
    expected_lines = [
        "usage: pls [-h] [-v]",
        "`pls` is a better `ls` for developers.",
        "positional arguments",
        "optional arguments",
    ]
    for line in expected_lines:
        assert line in proc.stdout
