from pathlib import Path

import pytest

from pls.models.node import Node
from pls.models.node_spec import NodeSpec

name_spec = NodeSpec(name="poetry.lock")
pattern_spec = NodeSpec(pattern=r"docker-compose\b")
extension_spec = NodeSpec(extension="py")


@pytest.mark.parametrize(
    "name, is_match",
    [
        ("poetry.lock", True),  # is exact match
        ("poetry", False),  # has missing chars
        ("poetry.lockfile", False),  # has extra chars
    ],
)
def test_name_spec_matches_exact_names(name: str, is_match: bool):
    test_node = Node(name, Path("."))
    assert name_spec.match(test_node) == is_match


@pytest.mark.parametrize(
    "name, is_match",
    [
        ("docker-compose", True),  # matches from start
        ("docker-composefile", False),  # doesn't match `\b`
        ("filedocker-compose", False),  # doesn't match start
    ],
)
def test_pattern_spec_matches_pattern_from_start(name: str, is_match: bool):
    test_node = Node(name, Path("."))
    assert pattern_spec.match(test_node) == is_match


@pytest.mark.parametrize(
    "name, is_match",
    [
        ("main.py", True),  # is exact match
        ("main.p", False),  # has missing chars
        ("main.pyc", False),  # has extra chars
    ],
)
def test_extension_spec_matches_exact_extension(name: str, is_match: bool):
    test_node = Node(name, Path("."))
    assert extension_spec.match(test_node) == is_match
