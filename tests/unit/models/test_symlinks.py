from __future__ import annotations

from typing import Callable

import pytest

from pls.enums.node_type import NodeType
from pls.models.node import Node
from pls.utils.strip_fmt import strip_formatting


@pytest.mark.parametrize(
    "name",
    [
        "symlink_dir",
        "symlink_file",
        "symlink_broken",
        "symlink_symlink_dir",
    ],
)
def test_symlink_has_correct_type(name: str, get_symlink: Callable[[str], Node]):
    symlink = get_symlink(name)
    assert symlink.node_type == NodeType.SYMLINK


def test_symlink_has_correct_type_char(get_symlink: Callable[[str], Node]):
    symlink = get_symlink("symlink_dir")
    assert symlink.type_char == "l"


@pytest.mark.parametrize(
    "name, dest",
    [
        ("symlink_dir", "dir"),
        ("symlink_file", "file"),
        ("symlink_broken", "broken"),
        ("symlink_symlink_dir", "symlink_dir"),
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
        ("symlink_dir", ["dir/"]),
        ("symlink_file", ["file"]),
        ("symlink_broken", ["broken⚠"]),
        ("symlink_symlink_dir", ["symlink_dir@", "dir/"]),
    ],
)
def test_symlink_has_dest_in_suffix(
    name: str, suffix_chain: list[str], get_symlink: Callable[[str], Node]
):
    symlink = get_symlink(name)
    suffix = f"@ → {' → '.join(suffix_chain)}"
    assert strip_formatting(symlink.formatted_suffix) == suffix
