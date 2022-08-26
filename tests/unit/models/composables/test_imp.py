from __future__ import annotations

from typing import Callable
from unittest.mock import patch

import pytest

from pls.globals import args
from pls.models.node import Node
from pls.models.node_spec import NodeSpec


@pytest.mark.parametrize("name", ["spec", ".spec"])
def test_nodes_have_importance_from_spec(name: str, get_node: Callable[[str], Node]):
    node = get_node(name)
    node.spec_comp.match(
        [NodeSpec(name="spec", importance=1), NodeSpec(".spec", importance=1)]
    )
    assert node.imp_comp.importance == 1


@pytest.mark.parametrize(
    "name, imp",
    [
        ("spec", 0),  # no leading dot, has spec without importance field
        ("no_spec", 0),  # no leading dot, no spec
        (".spec", -1),  # leading dot, has spec without importance field
        (".no_spec", -2),  # leading dot, no spec
    ],
)
def test_nodes_fallback_to_default_importance(
    name: str, imp: int, get_node: Callable[[str], Node]
):
    node = get_node(name)
    node.spec_comp.match([NodeSpec("spec"), NodeSpec(".spec")])
    assert node.imp_comp.importance == imp


@pytest.mark.parametrize(
    "imp, format_rules",
    [
        (-2, ["dim"]),
        (-1, ["dim"]),
        (0, []),
        (1, ["bold"]),
        (2, ["underline"]),
        (3, ["bold", "underline"]),
        (4, ["bold", "underline"]),
    ],
)
def test_nodes_have_correct_format_rules(
    imp: int, format_rules: list[str], get_node: Callable[[str], Node]
):
    node = get_node("file")
    node.spec_comp.specs.append(NodeSpec(importance=imp))
    assert node.imp_comp.format_rules == format_rules


@pytest.mark.parametrize(
    "args_all, is_visible",
    [
        (0, False),
        (1, True),
        (2, True),
    ],
)
def test_nodes_have_correct_visibility(
    args_all: int, is_visible: bool, get_node: Callable[[str], Node]
):
    node = get_node("file")
    node.spec_comp.specs.append(NodeSpec(importance=-2))
    with patch.multiple(args.args, all=args_all):
        assert node.imp_comp.is_visible == is_visible
