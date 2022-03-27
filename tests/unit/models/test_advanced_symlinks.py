from __future__ import annotations

from sys import platform
from typing import Callable

import pytest

from pls.enums.node_type import NodeType
from pls.models.node import Node
from pls.utils.strip_fmt import strip_formatting


if platform == "win32":
    pytest.skip(reason="Node types unsupported on Windows", allow_module_level=True)


@pytest.mark.parametrize(
    "name",
    [
        "symlink_fifo",
        "symlink_socket",
    ],
)
def test_symlink_has_correct_type(name: str, get_symlink: Callable[[str], Node]):
    symlink = get_symlink(name)
    assert symlink.node_type == NodeType.SYMLINK


@pytest.mark.parametrize(
    "name, dest",
    [
        ("symlink_fifo", "fifo"),
        ("symlink_socket", "socket"),
    ],
)
def test_symlinks_have_dest(
    name: str,
    dest: str,
    get_symlink: Callable[[str], Node],
    get_node: Callable[[str], Node],
):
    symlink = get_symlink(name)
    assert symlink.node_type == NodeType.SYMLINK  # ``node_type`` computes ``dest``
    assert symlink.dest_node.name == dest


@pytest.mark.parametrize(
    "name, suffix_chain",
    [
        ("symlink_fifo", ["fifo|"]),
        ("symlink_socket", ["socket="]),
    ],
)
def test_symlink_has_dest_in_suffix(
    name: str, suffix_chain: list[str], get_symlink: Callable[[str], Node]
):
    symlink = get_symlink(name)
    suffix = f"@ → {' → '.join(suffix_chain)}"
    assert strip_formatting(symlink.formatted_suffix) == suffix


def test_handles_cyclic_symlinks(cyclic_symlinks: tuple[Node, Node]):
    a, b = cyclic_symlinks
    assert strip_formatting(a.formatted_suffix) == "@ ↺ symlink_b"
    assert strip_formatting(b.formatted_suffix) == "@ ↺ symlink_a"
