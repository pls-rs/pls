from __future__ import annotations

import os
import subprocess
from pathlib import Path
from typing import Callable, Union


# pls/tests/e2e/utils.py
# ^ parents[2]
proj_dir = Path(__file__).parents[2]


def get_workbench(
    name: str,
    parent: Path = proj_dir,
    children: list[Union[str, Callable[[Path], None]]] = None,
) -> Path:
    """
    Create a workbench with the given name inside the given directory. If a list of
    children is provided

    - string values will be created as a file using ``touch``
    - callable values will be invoked with the workbench as the sole argument

    :param name: the name of the workbench directory
    :param parent: the path to the parent directory inside which to create the workbench
    :param children: the name or callable to create children inside the workbench
    :return: the newly created workbench directory
    """

    workbench = parent.joinpath(name)
    workbench.mkdir(mode=0o755)

    if children:
        for child in children:
            if callable(child):
                child(workbench)
            else:
                workbench.joinpath(child).touch(mode=0o644)

    return workbench


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
