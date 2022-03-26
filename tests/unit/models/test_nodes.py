from __future__ import annotations

from typing import Callable

import pytest

from pls.enums.node_type import NodeType
from pls.models.node import Node
from tests.unit.utils import strip_formatting


def test_handles_node_named_like_rich_formatting(get_node: Callable[[str], Node]):
    node = get_node("[red].py")
    assert node.name == r"\[red].py"  # Names like Rich markup are escaped.


@pytest.mark.parametrize(
    "name, node_type",
    [
        ("dir", NodeType.DIR),
        ("file", NodeType.FILE),
        ("broken", NodeType.UNKNOWN),
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
        ("dir", "/"),
        ("file", ""),
        ("broken", "⚠"),
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
        ("dir", "d"),
        ("file", "-"),
        ("broken", "?"),
    ],
)
def test_node_has_correct_type_char(
    name: str, type_char: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert strip_formatting(node.formatted_type_char) == type_char


@pytest.mark.parametrize(
    "name",
    ["dir", "file", "broken"],
)
def test_nodes_have_no_dest(name: str, get_node: Callable[[str], Node]):
    node = get_node(name)
    assert node.dest_node is None


@pytest.mark.parametrize(
    "name, format_left, format_right",
    [
        ("dir", "[cyan]", "[/]"),
        ("file", "", ""),
        ("broken", "[red]", "[/]"),
    ],
)
def test_node_has_correct_format_pair(
    name: str, format_left: str, format_right: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.format_pair == (format_left, format_right)


@pytest.mark.parametrize(
    "name",
    ["dir", "file"],
)
def test_non_broken_nodes_exist(name: str, get_node: Callable[[str], Node]):
    node = get_node(name)
    assert node.exists


def test_broken_node_does_not_exist(get_node: Callable[[str], Node]):
    node = get_node("broken")
    assert not node.exists
