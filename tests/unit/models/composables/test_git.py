from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

import pytest

from pls.globals import state
from pls.models.node import Node


@pytest.mark.parametrize(
    "name, git_root_wrt_workbench, path_wrt_git",
    [
        ("file", ".", Path("file")),
        ("file", "..", Path("workbench/file")),
    ],
)
def test_nodes_have_correct_path_wrt_git_root(
    name: str,
    git_root_wrt_workbench: str,
    path_wrt_git: Path,
    workbench: Path,
    get_node: Callable[[str], Node],
):
    node = get_node(name)
    with patch.multiple(
        state.state,
        git_root=workbench.joinpath(git_root_wrt_workbench).resolve(),
        git_status_map={},
    ):
        node = Node(name=name, path=node.path.absolute())
        assert node.git_comp.path_wrt_git == path_wrt_git


def test_nodes_have_no_path_wrt_git_when_not_repo(get_node: Callable[[str], Node]):
    with patch.multiple(state.state, git_root=None, git_status_map={}):
        node = get_node("file")
    assert node.git_comp.path_wrt_git is None


@pytest.mark.parametrize(
    "name, git_status",
    [
        ("file_a", "XY"),
        ("file_b", None),
    ],
)
def test_nodes_have_correct_git_status(
    name: str,
    git_status: str,
    workbench: Path,
    get_node: Callable[[str], Node],
):
    with patch.multiple(
        state.state,
        git_root=workbench,
        git_status_map={Path("file_a"): "XY"},
    ):
        node = get_node(name)
        assert node.git_comp.git_status == git_status


def test_nodes_have_no_git_status_when_not_repo(get_node: Callable[[str], Node]):
    with patch.multiple(state.state, git_root=None, git_status_map={}):
        node = get_node("file")
    assert node.git_comp.git_status is None


@pytest.mark.parametrize(
    "name, cells",
    [
        ("file_a", {"git": "XY"}),
        ("file_b", {"git": "  "}),
    ],
)
def test_nodes_have_correct_cells(
    name: str, cells: dict, workbench: Path, get_node: Callable[[str], Node]
):
    with patch.multiple(
        state.state, git_root=workbench, git_status_map={Path("file_a"): "XY"}
    ):
        node = get_node(name)
    assert node.git_comp.cells == cells


@pytest.mark.parametrize(
    "name, format_rules",
    [
        ("file_a", ["dim"]),
        ("file_b", []),
    ],
)
def test_nodes_have_correct_format_rules(
    name: str, format_rules: list[str], workbench: Path, get_node: Callable[[str], Node]
):
    with patch.multiple(
        state.state, git_root=workbench, git_status_map={Path("file_a"): "!!"}
    ):
        node = get_node(name)
    assert node.git_comp.format_rules == format_rules
