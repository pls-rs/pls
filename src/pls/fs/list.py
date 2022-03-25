from __future__ import annotations

import os
from pathlib import Path
from typing import Optional

from pls.enums.node_type import NodeType
from pls.globals import args, console
from pls.models.node import Node


def sort_key(node: Node) -> tuple:
    """
    Map a ``Node`` instance to a string that represents it. This string is used
    to sort a list of ``Node`` instances.

    :param node: the node item for which to get the sorting key
    :return: the value to use as the sort key for the node
    """

    key = node.sort_keys[args.args.sort.rstrip("-")]
    is_reversed = args.args.sort.endswith("-")
    if args.args.dirs_first:
        if is_reversed:
            type_key = 1 if node.node_type == NodeType.DIR else 0
        else:
            type_key = 0 if node.node_type == NodeType.DIR else 1
        return type_key, key, node.sort_keys["name"]
    return key, node.name


def passes_filters(node: Node) -> bool:
    """
    Determine whether the given node fulfils the filtering criteria.

    :param node: the node to test against the filters
    :return: ``True`` if the node passes the filters, ``False`` otherwise
    """

    if args.args.exclude and args.args.exclude.match(node.name) is not None:
        return False
    if args.args.only and args.args.only.match(node.name) is None:
        return False
    return True


def parse_node(node_name: str) -> Optional[Node]:
    """
    Parse the node name into a ``Node`` instance. Most of the heavy lifting is
    handled in the ``Node`` class definition itself.

    :param node_name: the name of a node inside the working directory
    :return: a ``Node`` instance
    """

    node_path: Path = args.args.directory.joinpath(node_name)

    if node_path.is_dir():
        if not args.args.dirs:
            return None
    else:  # is some kind of file
        if not args.args.files:
            return None

    return Node(name=node_name, path=node_path)


def read_input() -> tuple[dict[str, Node], list[Node]]:
    """
    Get a list of all directories and files in the given directory.

    :return: the list of directories and files inside the given directory
    """

    all_nodes = os.listdir(args.args.directory)

    node_map = {}
    node_list = []

    if not all_nodes:
        console.console.print(
            f"There are no files or folders in [repr.path]{args.args.directory}[/].",
            highlight=False,
        )
    else:
        node_map = {
            parsed_node.name: parsed_node
            for node in all_nodes
            if (parsed_node := parse_node(node)) and passes_filters(parsed_node)
        }
        node_list = list(node_map.values())
        node_list.sort(key=sort_key, reverse=args.args.sort.endswith("-"))

    return node_map, node_list
