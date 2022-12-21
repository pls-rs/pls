from __future__ import annotations

import os
import subprocess
from pathlib import Path
from typing import List, Optional, Tuple, Union

# pls/tests/e2e/utils.py
# ^ parents[2]
proj_dir = Path(__file__).parents[2]

BenchGen = Union[str, Tuple[str, List["BenchGen"]]]  # type:ignore


def get_workbench(
    bench_gen: BenchGen = "workbench", parent_path: Path = proj_dir
) -> Path:
    """
    Create a workbench with the entire directory structure as specified by the
    ``bench_gen`` argument. The ``bench_gen`` argument can be one of two types:

    - ``str``: to create a file
    - ``tuple[str, list[BenchGen]]``: to create a directory and recursively populate it

    :param bench_gen: the directory structure as ``tuple`` and ``list`` instances
    :param parent_path: the directory in which to create the workbench
    :return: the path to the created workbench
    """

    curr_name: str
    children_gen: Optional[list[BenchGen]] = None
    if isinstance(bench_gen, tuple):
        curr_name, children_gen = bench_gen
    else:  # type(bench_gen) == str
        curr_name = bench_gen

    curr_path = parent_path.joinpath(curr_name)
    if children_gen is None:
        curr_path.touch(mode=0o644)
    else:
        curr_path.mkdir(mode=0o755)

    if children_gen:
        for child_gen in children_gen:
            get_workbench(child_gen, curr_path)

    return curr_path


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
