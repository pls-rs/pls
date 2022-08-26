from __future__ import annotations

import os
import shutil
import socket
from pathlib import Path
from sys import platform
from typing import Literal

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
    def _get_node(name: str) -> Node:
        if name == "tty":
            name = "/dev/tty"
            return Node(name=name, path=Path(name))

        if name == "disk0":  # General disk name on macOS
            name = "/dev/disk0"
            return Node(name=name, path=Path(name))

        if name == "sda1":  # General disk name for Linux
            name = "/dev/sda1"
            return Node(name=name, path=Path(name))

        path = workbench.joinpath(name)
        if not path.exists():
            if name == "broken":
                pass
            elif name == "dir":
                path.mkdir(mode=0o755)
            elif name == "fifo":
                os.mkfifo(path, mode=0o644)
            elif name == "socket":
                af = socket.AF_INET if platform == "win32" else socket.AF_UNIX
                socket.socket(af).bind(str(path.resolve()))
            elif name.startswith("symlink_"):
                dest = name[8:]
                dest_node = _get_node(dest)
                path.unlink(missing_ok=True)  # symlink is recreated to match target
                path.symlink_to(dest_node._name, target_is_directory=dest == "dir")
            else:  # name == "file" or anything else
                path.touch(mode=0o644)
        return Node(name=name, path=path)

    return _get_node


@pytest.fixture(scope=scope)
def cyclic_symlinks(workbench: Path):
    names = [f"symlink_{name}" for name in ["a", "b"]]
    paths = [workbench.joinpath(name) for name in names]
    for index, path in enumerate(paths):
        path.unlink(missing_ok=True)
        path.symlink_to(names[(index + 1) % len(names)])
    nodes = [Node(name=name, path=path) for name, path in zip(names, paths)]
    return nodes


@pytest.fixture(scope=scope)
def get_cyclic_symlinks(workbench: Path):
    def _get_cyclic_symlinks(count: int) -> list[Node]:
        names = [f"symlink_{i}" for i in range(count)]
        paths = [workbench.joinpath(name) for name in names]
        for index, path in enumerate(paths):
            path.unlink(missing_ok=True)
            path.symlink_to(names[(index + 1) % len(names)])
        nodes = [Node(name=name, path=path) for name, path in zip(names, paths)]
        return nodes

    return _get_cyclic_symlinks
