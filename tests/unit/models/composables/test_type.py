from __future__ import annotations

from sys import platform
from typing import Callable
from unittest.mock import patch

import pytest

from pls.config import constants
from pls.config.constants import get_constants
from pls.data.utils import internal_yml_path
from pls.enums.node_type import NodeType, SymlinkState
from pls.models.node import Node
from pls.utils.strip_fmt import strip_formatting


skip_if_win32 = pytest.mark.skipif(
    platform == "win32", reason="Not supported on Windows"
)
skip_unless_linux = pytest.mark.skipif(
    platform != "linux", reason="Only supported on Linux"
)
skip_unless_darwin = pytest.mark.skipif(
    platform != "darwin", reason="Only supported on macOS"
)


@pytest.mark.parametrize(
    "name, node_type",
    [
        ("symlink_file", NodeType.SYMLINK),
        ("dir", NodeType.DIR),
        ("file", NodeType.FILE),
        pytest.param("fifo", NodeType.FIFO, marks=skip_if_win32),
        pytest.param("socket", NodeType.SOCKET, marks=skip_if_win32),
        pytest.param("tty", NodeType.CHAR_DEVICE, marks=skip_if_win32),
        pytest.param("disk0", NodeType.BLOCK_DEVICE, marks=skip_unless_darwin),
        pytest.param("sda1", NodeType.BLOCK_DEVICE, marks=skip_unless_linux),
    ],
)
def test_nodes_have_correct_node_type(
    name: str, node_type: NodeType, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.type_comp.node_type == node_type


@pytest.mark.parametrize(
    "name",
    [
        "dir",
        "file",
        pytest.param("fifo", marks=skip_if_win32),
        pytest.param("socket", marks=skip_if_win32),
        pytest.param("tty", marks=skip_if_win32),
        pytest.param("disk0", marks=skip_unless_darwin),
        pytest.param("sda1", marks=skip_unless_linux),
    ],
)
def test_non_symlinks_have_no_dest_node(name: str, get_node: Callable[[str], Node]):
    node = get_node(name)
    assert node.type_comp.dest_node is None


@pytest.mark.parametrize(
    "name, dest_node_type",
    [
        ("symlink_dir", NodeType.DIR),
        ("symlink_file", NodeType.FILE),
        pytest.param("symlink_fifo", NodeType.FIFO, marks=skip_if_win32),
        pytest.param("symlink_socket", NodeType.SOCKET, marks=skip_if_win32),
        pytest.param("symlink_tty", NodeType.CHAR_DEVICE, marks=skip_if_win32),
        pytest.param("symlink_disk0", NodeType.BLOCK_DEVICE, marks=skip_unless_darwin),
        pytest.param("symlink_sda1", NodeType.BLOCK_DEVICE, marks=skip_unless_linux),
    ],
)
def test_symlinks_have_dest_node(
    name: str, dest_node_type: NodeType, get_node: Callable[[str], Node]
):
    node = get_node(name)
    dest_node = node.type_comp.dest_node
    assert isinstance(dest_node, Node)
    assert dest_node.type_comp.node_type == dest_node_type


def test_symlinks_escape_dest_node_name(get_node: Callable[[str], Node]):
    node = get_node("symlink_[red]dest")
    dest_node = node.type_comp.dest_node
    assert isinstance(dest_node, Node)
    assert dest_node.name == r"\[red]dest"  # Names like Rich markup are escaped.


def test_chained_symlinks_have_chained_dest_nodes(get_node: Callable[[str], Node]):
    curr_node = get_node("symlink_symlink_file")
    for node_type in [NodeType.SYMLINK, NodeType.SYMLINK, NodeType.FILE]:
        curr_type = curr_node.type_comp.node_type
        assert curr_type == node_type
        if isinstance(node := curr_node.type_comp.dest_node, Node):
            curr_node = node


@skip_if_win32
@pytest.mark.parametrize("count", [2, 3])
def test_cyclic_symlinks_are_marked_as_loops(
    count: int, get_cyclic_symlinks: Callable[[int], list[Node]]
):
    nodes = get_cyclic_symlinks(count)
    for node in nodes:
        assert node.type_comp.symlink_state == SymlinkState.LOOP


@skip_if_win32
@pytest.mark.parametrize("count", [2, 3])
def test_cyclic_symlinks_have_str_dest(
    count: int, get_cyclic_symlinks: Callable[[int], list[Node]]
):
    nodes = get_cyclic_symlinks(count)
    for node in nodes:
        assert isinstance(node.type_comp.dest_node, str)


@pytest.mark.parametrize(
    "name, depth",
    [
        ("symlink_broken", 1),
        ("symlink_symlink_broken", 2),
    ],
)
def test_broken_symlinks_are_marked_as_broken(
    name: str, depth: int, get_node: Callable[[str], Node]
):
    node = get_node(name)
    for _ in range(depth - 1):
        dest_node = node.type_comp.dest_node
        if isinstance(dest_node, Node):
            node = dest_node
    assert node.type_comp.symlink_state == SymlinkState.BROKEN


@pytest.mark.parametrize(
    "name, depth",
    [
        ("symlink_broken", 1),
        ("symlink_symlink_broken", 2),
    ],
)
def test_broken_symlinks_have_str_dest(
    name: str, depth: int, get_node: Callable[[str], Node]
):
    node = get_node(name)
    for _ in range(depth - 1):
        dest_node = node.type_comp.dest_node
        if isinstance(dest_node, Node):
            node = dest_node
    assert isinstance(node.type_comp.dest_node, str)


@pytest.mark.parametrize(
    "name, type_char, suffix_char, icon",
    [
        ("symlink_file", "l", "@", None),
        ("dir", "d", "/", "folder"),
        ("file", "-", None, None),
        pytest.param("fifo", "p", "|", None, marks=skip_if_win32),
        pytest.param("socket", "s", "=", None, marks=skip_if_win32),
        pytest.param("tty", "c", None, None, marks=skip_if_win32),
        pytest.param("disk0", "b", None, None, marks=skip_unless_darwin),
        pytest.param("sda1", "b", None, None, marks=skip_unless_linux),
    ],
)
def test_nodes_have_correct_type_constants(
    name: str,
    type_char: str,
    suffix_char: str,
    icon: str,
    get_node: Callable[[str], Node],
):
    with patch.object(
        constants, "constants", get_constants([internal_yml_path("constants.yml")])
    ):
        node = get_node(name)

    if node_type_char := node.type_comp.type_char:
        node_type_char = strip_formatting(node_type_char)
    assert node_type_char == type_char

    if node_suffix_char := node.type_comp.suffix_char:
        node_suffix_char = strip_formatting(node_suffix_char)
    assert node_suffix_char == suffix_char

    assert node.type_comp.icon == icon


@pytest.mark.parametrize(
    "name, suffix",
    [
        ("symlink_dir", "@ → dir/"),
        ("symlink_file", "@ → file"),
        ("symlink_symlink_dir", "@ → symlink_dir@ → dir/"),
        pytest.param("symlink_fifo", "@ → fifo|", marks=skip_if_win32),
        pytest.param("symlink_socket", "@ → socket=", marks=skip_if_win32),
        pytest.param("symlink_tty", "@ → /dev/tty", marks=skip_if_win32),
        pytest.param("symlink_disk0", "@ → /dev/disk0", marks=skip_unless_darwin),
        pytest.param("symlink_sda1", "@ → /dev/sda1", marks=skip_unless_linux),
    ],
)
def test_symlink_suffix_shows_target(
    name: str, suffix: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert strip_formatting(node.type_comp.display_suffix) == suffix


@pytest.mark.parametrize(
    "name, suffix",
    [
        ("symlink_broken", "@ ↝ broken⚠"),
        ("symlink_symlink_broken", "@ → symlink_broken@ ↝ broken⚠"),
    ],
)
def test_suffix_indicates_broken_symlinks(
    name, suffix, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert strip_formatting(node.type_comp.display_suffix) == suffix


@skip_if_win32
@pytest.mark.parametrize("count", [2, 3])
def test_suffix_indicates_cyclic_symlinks(
    count: int, get_cyclic_symlinks: Callable[[int], list[Node]]
):
    nodes = get_cyclic_symlinks(count)
    for i in range(count):
        assert (
            strip_formatting(nodes[i].type_comp.display_suffix)
            == f"@ ↺ symlink_{(i + 1) % count}"
        )
