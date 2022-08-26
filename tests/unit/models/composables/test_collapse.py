from __future__ import annotations

from typing import Callable
from unittest.mock import patch

import pytest

from pls.globals import args
from pls.models.node import Node
from pls.models.node_spec import NodeSpec
from pls.models.tree import Tree


@pytest.mark.parametrize(
    "name, spec, parent_name",
    [
        ("a", NodeSpec(name="a", collapse={"name": "c"}), "c"),
        ("b", NodeSpec(name="b", collapse={"name": "a"}), "a"),
    ],
)
def test_com_finds_main_by_matching_names(
    name: str, spec: NodeSpec, parent_name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    node.spec_comp.specs.append(spec)
    parent_node = get_node(parent_name)

    all_nodes = {char: get_node(char) for char in ["a", "b", "c"]}
    node.collapse_comp.find_main(all_nodes)
    assert node.parent == parent_node


@pytest.mark.parametrize(
    "name, spec, parent_name",
    [
        ("a.auto", NodeSpec(extension="auto", collapse={"extension": "man"}), "a.man"),
        ("a.ai", NodeSpec(extension="ai", collapse={"extension": "auto"}), "a.auto"),
    ],
)
def test_comp_finds_main_by_matching_extensions(
    name: str, spec: NodeSpec, parent_name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    node.spec_comp.specs.append(spec)
    parent_node = get_node(parent_name)

    all_nodes = {f"a.{ext}": get_node(f"a.{ext}") for ext in ["man", "auto", "ai"]}
    node.collapse_comp.find_main(all_nodes)
    assert node.parent == parent_node


@pytest.mark.parametrize(
    "args_collapse, is_visible",
    [
        (0, True),
        (1, True),
        (2, False),
    ],
)
def test_sub_nodes_have_correct_visibility_when(
    args_collapse: int, is_visible: bool, get_node: Callable[[str], Node]
):
    parent = get_node("parent")
    child = get_node("child")
    if args_collapse:
        Tree.link(parent, child)
    with patch.multiple(args.args, collapse=args_collapse):
        assert child.collapse_comp.is_visible == is_visible


@pytest.mark.parametrize("args_collapse", [0, 1, 2])
def test_dom_nodes_are_always_visible(
    args_collapse: int, get_node: Callable[[str], Node]
):
    node = get_node("file")
    with patch.multiple(args.args, collapse=args_collapse):
        assert node.collapse_comp.is_visible
