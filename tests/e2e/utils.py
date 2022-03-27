from __future__ import annotations

import subprocess
from pathlib import Path


# pls/tests/e2e/utils.py
# ^ parents[2]
proj_dir = Path(__file__).parents[2]


def run_pls(args: list[str] = None) -> subprocess.CompletedProcess:
    """
    Run ``pls`` under Poetry's virtual environment.

    :param args: the list of arguments to pass to ``pls``
    :return: the ``CompletedProcess`` instance
    """

    if args is None:
        args = []
    cmd = ["poetry", "run", "pls", *args]
    proc = subprocess.run(
        cmd,
        check=True,
        cwd=proj_dir,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    return proc
