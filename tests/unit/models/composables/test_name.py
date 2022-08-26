from __future__ import annotations

from typing import Callable
from unittest.mock import patch

import pytest

from pls.globals import args
from pls.models.format_rules import FormatRules
from pls.models.node import Node


@pytest.mark.parametrize(
    "name, pure_name, canonical_name",
    [
        ("file", "file", "file"),
        (".file", "file", "file"),  # Pure name removes leading dot.
        ("..file", ".file", ".file"),  # Only 1 leading dot is removed.
        (".fIlE", "fIlE", "file"),  # Canonical name is case-insensitive.
    ],
)
def test_nodes_have_correct_pure_and_canonical_name(
    name: str, pure_name: str, canonical_name: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.name_comp.pure_name == pure_name
    assert node.name_comp.canonical_name == canonical_name


@pytest.mark.parametrize(
    "name, ext",
    [
        ("file", None),
        (".file", None),  # Leading dot is not considered for extension.
        ("file.txt", "txt"),
        ("file.tar.gz", "gz"),  # Only last fragment is considered the extension.
    ],
)
def test_nodes_have_correct_extension(
    name: str, ext: str, get_node: Callable[[str], Node]
):
    node = get_node(name)
    assert node.name_comp.ext == ext


@pytest.mark.parametrize(
    "name, args_align, format_rules, display_name",
    [
        ("file", True, [], " file"),
        (
            "file",
            True,
            ["underline"],
            " [underline]file[/]",
        ),  # formatting is not applied to padding space
        ("file", False, [], "file"),
        (".file", True, [], "[dim].[/]file"),  # Leading dot is dimmed if aligned.
        (".file", False, [], ".file"),  # Leading dot is not dimmed if not aligned.
        (
            ".file",
            True,
            ["underline"],
            "[underline][dim].[/]file[/]",
        ),  # Leading dot is formatted.
    ],
)
def test_nodes_have_correct_display_name(
    name: str,
    args_align: bool,
    format_rules: list[str],
    display_name: str,
    get_node: Callable[[str], Node],
):
    node = get_node(name)
    with patch.multiple(node, format_rules=FormatRules(format_rules)), patch.multiple(
        args.args, align=args_align
    ):
        assert node.name_comp.display_name == display_name
