from __future__ import annotations

from sys import platform
from typing import Callable

import pytest

from pls.enums.node_type import NodeType
from pls.models.node import Node
from tests.unit.utils import strip_formatting


if platform == "win32":
    pytest.skip(reason="Node types unsupported on Windows", allow_module_level=True)


@pytest.mark.parametrize(
    "name, node_type",
    [
        ("fifo", NodeType.FIFO),
        ("socket", NodeType.SOCKET),
    ],
)
def test_node_has_correct_type(
    name: str, node_type: NodeType, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.node_type == node_type


@pytest.mark.parametrize(
    "name, suffix",
    [
        ("fifo", "|"),
        ("socket", "="),
    ],
)
def test_node_has_correct_suffix(
    name: str, suffix: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert strip_formatting(node.formatted_suffix) == suffix


@pytest.mark.parametrize(
    "name, type_char",
    [
        ("fifo", "p"),
        ("socket", "s"),
    ],
)
def test_node_has_correct_type_char(
    name: str, type_char: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.type_char == type_char


@pytest.mark.parametrize(
    "name",
    ["fifo", "socket"],
)
def test_nodes_have_no_dest(name: str, get_node: Callable[[str], Node]):
    node = get_node(name)
    assert node.dest_node is None


@pytest.mark.parametrize(
    "name, format_left, format_right",
    [
        ("fifo", "", ""),
        ("socket", "", ""),
    ],
)
def test_node_has_correct_format_pair(
    name: str, format_left: str, format_right: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.format_pair == (format_left, format_right)


@pytest.mark.parametrize(
    "name",
    ["fifo", "socket"],
)
def test_non_broken_nodes_exist(name: str, get_node: Callable[[str], Node]):
    node = get_node(name)
    assert node.exists
