from __future__ import annotations

from unittest.mock import patch

import pytest

from pls.globals import args
from pls.models.tree import Tree


class Entry(Tree):
    def __init__(self, name, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.name = name


@pytest.fixture
def tree() -> dict[str, Entry]:
    """
    a
    ├─ b
    │  └─ c
    └─ d
       ├─ e
       └─ f
    """
    entries: dict[str, Entry] = {}
    for node_name in list("abcdef"):
        entries[node_name] = Entry(node_name)

    Tree.link(entries["a"], entries["b"], entries["d"])
    Tree.link(entries["b"], entries["c"])
    Tree.link(entries["d"], entries["e"], entries["f"])

    entries["a"].set_sub_pre_shapes()
    return entries


@pytest.mark.parametrize(
    "align, prefix_map",
    [
        (
            True,
            {
                "a": " ",
                "b": " ├─",
                "c": " │ └─",
                "d": " └─",
                "e": "   ├─",
                "f": "   └─",
            },
        ),
        (
            False,
            {
                "a": "",
                "b": "├─",
                "c": "│ └─",
                "d": "└─",
                "e": "  ├─",
                "f": "  └─",
            },
        ),
    ],
)
def test_tree_nodes_have_correct_shapes(
    align: bool, prefix_map: dict[str, str], tree: dict[str, Entry]
):
    with patch.multiple(args.args, align=align):
        for name, entry in tree.items():
            print([name, entry.tree_prefix])
            assert entry.tree_prefix == prefix_map[name]
