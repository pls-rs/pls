from __future__ import annotations

from sys import platform
from typing import Callable
from unittest.mock import patch

import pytest

from pls.globals import args
from pls.models.node import Node


skip_if_win32 = pytest.mark.skipif(
    platform == "win32", reason="Does not exist on Windows"
)
skip_if_linux = pytest.mark.skipif(
    platform == "linux", reason="Does not exist on Linux"
)
skip_if_darwin = pytest.mark.skipif(
    platform == "darwin", reason="Does not exist on macOS"
)


def test_stat_is_none_for_non_existent_files(get_node: Callable[[str], Node]):
    node = get_node("broken")
    assert node.stat_comp.stat is None


@pytest.mark.parametrize(
    "name",
    ["dir", "file", "symlink_file", "symlink_broken"],
)
def test_stat_is_not_none_for_existing_nodes(
    name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.stat_comp.stat is not None


@skip_if_win32
@pytest.mark.parametrize(
    "name",
    [
        "fifo",
        "socket",
        "tty",
        pytest.param("disk0", marks=skip_if_linux),
        pytest.param("sda1", marks=skip_if_darwin),
    ],
)
def test_stat_is_not_none_for_existing_advanced_nodes(
    name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.stat_comp.stat is not None


def test_cells_are_blank_for_non_existent_files(get_node: Callable[[str], Node]):
    node = get_node("broken")
    assert node.stat_comp.cells == {}


@pytest.mark.parametrize(
    "details",
    [
        ["inode"],
        ["inode", "links"],
        [
            "inode",
            "links",
            "perms",
            "user",
            "group",
            "size",
            "btime",
            "ctime",
            "mtime",
            "atime",
        ],
    ],
)
@pytest.mark.parametrize(
    "name",
    [
        "dir",
        "file",
        pytest.param("fifo", marks=skip_if_win32),
        pytest.param("socket", marks=skip_if_win32),
        pytest.param("tty", marks=skip_if_win32),
        pytest.param("disk0", marks=[skip_if_win32, skip_if_linux]),
        pytest.param("sda1", marks=[skip_if_win32, skip_if_darwin]),
    ],
)
def test_specified_cells_are_populated_for_existing_nodes(
    details: list[str], name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    with patch.multiple(args.args, details=details):
        for key in details:
            assert key in node.stat_comp.cells


@pytest.mark.parametrize(
    "name",
    ["dir", "file", "symlink_file", "symlink_broken"],
)
def test_keys_are_populated_for_existing_nodes(
    name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    for key in [
        "inode",
        "links",
        "size",
        "btime",
        "ctime",
        "mtime",
        "atime",
    ]:
        assert key in node.stat_comp.keys


@skip_if_win32
@pytest.mark.parametrize(
    "name",
    [
        "fifo",
        "socket",
        "tty",
        pytest.param("disk0", marks=skip_if_linux),
        pytest.param("sda1", marks=skip_if_darwin),
    ],
)
def test_keys_are_populated_for_existing_advanced_nodes(
    name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    for key in [
        "inode",
        "links",
        "size",
        "btime",
        "ctime",
        "mtime",
        "atime",
    ]:
        assert key in node.stat_comp.keys
