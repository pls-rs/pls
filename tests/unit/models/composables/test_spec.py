from __future__ import annotations

from typing import Callable

import pytest

from pls.models.node import Node
from pls.models.node_spec import NodeSpec


@pytest.fixture
def specs() -> list[NodeSpec]:
    return [
        NodeSpec(name="file"),
        NodeSpec(extension="yml", color="yellow"),
        NodeSpec(pattern=r".pls.(yml|json)", icon="pls", color="red"),
    ]


@pytest.mark.parametrize(
    "name, match_indices",
    [
        ("file", [0]),
        (".pls.yml", [1, 2]),
    ],
)
def test_comp_matches_specs(
    name: str,
    match_indices: list[int],
    specs: list[NodeSpec],
    get_node: Callable[[str], Node],
):
    node = get_node(name)
    node.spec_comp.match(specs)
    for idx in match_indices:
        assert specs[idx] in node.spec_comp.specs


@pytest.mark.parametrize(
    "attr, value",
    [
        ("color", "yellow"),  # matched in first spec
        ("icon", "pls"),  # matched in second spec
    ],
)
def test_comp_gets_first_matching_attr(
    attr: str, value: str, specs: list[NodeSpec], get_node: Callable[[str], Node]
):
    node = get_node(".pls.yml")
    node.spec_comp.match(specs)
    assert node.spec_comp.attr(attr) == value


def test_comp_gets_all_matching_attrs(
    specs: list[NodeSpec], get_node: Callable[[str], Node]
):
    node = get_node(".pls.yml")
    node.spec_comp.match(specs)
    assert node.spec_comp.attr("color", True) == ["yellow", "red"]
