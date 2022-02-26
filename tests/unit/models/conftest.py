from __future__ import annotations

import os
import shutil
import socket
from pathlib import Path
from typing import Callable, Literal

import pytest

from pls.models.node import Node


scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def workbench():
    conftest_path = Path(__file__)
    workbench = conftest_path.parent.joinpath("workbench")
    workbench.mkdir(mode=0o755)

    yield workbench

    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def get_node(workbench: Path):
    def _get_node(name: str):
        path = workbench.joinpath(name)
        if not path.exists():
            if name == "dir":
                path.mkdir(mode=0o755)
            elif name == "file":
                path.touch(mode=0o644)
            elif name == "fifo":
                os.mkfifo(path, mode=0o644)
            elif name == "socket":
                socket.socket(socket.AF_UNIX).bind(str(path.resolve()))
            elif name == "broken":
                pass
        return Node(name=name, path=path)

    return _get_node


@pytest.fixture(scope=scope)
def get_symlink(workbench: Path, get_node: Callable[[str], Node]):
    def _get_symlink(name: str):
        dest = name.replace("symlink_", "", 1)
        path = workbench.joinpath(name)
        try:
            path.lstat()
        except FileNotFoundError:
            path.symlink_to(dest, target_is_directory=dest == "dir")

        if dest.startswith("symlink_"):
            _get_symlink(dest)
        else:
            get_node(dest)

        return Node(name=name, path=path)

    return _get_symlink


@pytest.fixture(scope=scope)
def cyclic_symlinks(workbench: Path):
    names = [f"symlink_{name}" for name in ["a", "b"]]
    paths = [workbench.joinpath(name) for name in names]
    for index, path in enumerate(paths):
        try:
            path.lstat()
        except FileNotFoundError:
            path.symlink_to(names[(index + 1) % len(names)])
    nodes = [Node(name=name, path=path) for name, path in zip(names, paths)]
    return nodes
