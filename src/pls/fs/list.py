from __future__ import annotations

import os
from pathlib import Path
from typing import Optional

from rich.console import Console

from pls.args import args
from pls.enums.node_type import NodeType
from pls.models.node import Node


console = Console()


def sort_key(node: Node) -> tuple:
    """
    Map a ``Node`` instance to a string that represents it. This string is used
    to sort a list of ``Node`` instances.

    :param node: the node item to reduce to a string
    :return: a string representative of the node
    """

    key = node.sort_keys[args.sort.rstrip("-")]
    is_reversed = args.sort.endswith("-")
    if not args.no_dirs_first:
        if is_reversed:
            type_key = 1 if node.node_type == NodeType.DIR else 0
        else:
            type_key = 0 if node.node_type == NodeType.DIR else 1
        return type_key, key, node.sort_keys["name"]
    return key, node.name


def parse_node(node_name: str) -> Optional[Node]:
    """
    Parse the node name into a ``Node`` instance. Most of the heavy lifting is
    handled in the ``Node`` class definition itself.

    :param node_name: the name of a node inside the working directory
    :return: a ``Node`` instance
    """

    node_path: Path = args.directory.joinpath(node_name)

    if node_path.is_dir():
        if args.no_dirs:
            return None
    else:  # is some kind of file
        if args.no_files:
            return None

    return Node(node_name, path=node_path)


def read_input() -> tuple[dict[str, Node], list[Node]]:
    """
    Get a list of all directories and files in the given directory.

    :return: the list of directories and files inside the given directory
    """

    all_nodes = os.listdir(args.directory)

    node_map = {}
    node_list = []

    if not all_nodes:
        console.print(
            f"There are no files or folders in [bold]{args.directory}[/bold].",
            highlight=False,
        )
    else:
        node_map = {
            parsed_node.name: parsed_node
            for node in all_nodes
            if (parsed_node := parse_node(node)) is not None
        }
        node_list = list(node_map.values())
        node_list.sort(key=sort_key, reverse=args.sort.endswith("-"))

    return node_map, node_list
