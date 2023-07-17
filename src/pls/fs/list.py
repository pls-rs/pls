from __future__ import annotations

import os
from pathlib import Path
from typing import Optional

from pls.globals import args, console
from pls.models.node import Node


def passes_name_filters(name: str) -> bool:
    """
    Determine whether the given node fulfils the filtering criteria.

    :param name: the name of the node to test against the filters
    :return: ``True`` if the node passes the filters, ``False`` otherwise
    """

    if args.args.exclude and args.args.exclude.match(name) is not None:
        return False
    if args.args.only and args.args.only.match(name) is None:
        return False
    return True


def parse_node(parent_path: Path, node_name: str) -> Optional[Node]:
    """
    Parse the node name into a ``Node`` instance. Most of the heavy lifting is
    handled in the ``Node`` class definition itself.

    :param parent_path: the path to the parent of the node
    :param node_name: the name of a node inside the working directory
    :return: a ``Node`` instance
    """

    node_path: Path = parent_path.joinpath(node_name)
    return Node(name=node_name, path=node_path)


def read_input(arg_path: Path) -> tuple[dict[str, Node], list[Node]]:
    """
    Get a list of all directories and files in the given directory.

    :return: the list of directories and files inside the given directory
    """

    if arg_path.is_dir():
        parent_path = arg_path
        try:
            all_nodes = os.listdir(arg_path)
        except (OSError, PermissionError):
            console.console.print(
                f"Permission denied for [repr.path]{arg_path}[/].",
                highlight=False,
            )
            return {}, []
    else:
        parent_path = arg_path.parent
        all_nodes = [arg_path.name]

    if not all_nodes:
        console.console.print(
            f"There are no files or folders in [repr.path]{arg_path}[/].",
            highlight=False,
        )
        return {}, []

    node_map = {
        node: Node(name=node, path=parent_path.joinpath(node))
        for node in all_nodes
        if passes_name_filters(node)
    }
    node_list = list(node_map.values())
    sort_fields = args.args.sort
    for field in reversed(sort_fields):
        item = field.rstrip("-")
        node_list.sort(
            key=lambda node: node.sort_keys[item],
            reverse=field.endswith("-"),
        )

    return node_map, node_list
