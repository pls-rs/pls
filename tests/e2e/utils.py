from __future__ import annotations

import os
import subprocess
from pathlib import Path


# pls/tests/e2e/utils.py
# ^ parents[2]
proj_dir = Path(__file__).parents[2]


def run_pls(pls_args: list[str] = None, **kwargs) -> subprocess.CompletedProcess:
    """
    Run ``pls`` under Poetry's virtual environment.

    :param pls_args: the list of arguments to pass to ``pls``
    :param kwargs: the list of arguments to pass to the ``run`` command
    :return: the ``CompletedProcess`` instance
    """

    if pls_args is None:
        pls_args = []
    cmd = ["poetry", "run", "pls", *pls_args]

    run_kwargs = {
        "check": True,
        "cwd": proj_dir,
        "capture_output": True,
        "text": True,
        "encoding": "utf-8",
    }

    if "env" in kwargs:
        env = os.environ.copy()
        env.update(kwargs.pop("env"))
        run_kwargs["env"] = env

    run_kwargs.update(kwargs)

    proc = subprocess.run(cmd, **run_kwargs)  # type: ignore
    return proc
