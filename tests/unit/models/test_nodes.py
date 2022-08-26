from __future__ import annotations

from typing import Callable

from pls.models.node import Node


def test_handles_node_named_like_rich_formatting(get_node: Callable[[str], Node]):
    node = get_node("[red].py")
    assert node.name == r"\[red].py"  # Names like Rich markup are escaped.
