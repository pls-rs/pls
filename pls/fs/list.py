from __future__ import annotations

import multiprocessing
import os

from rich.console import Console

from pls.args import args
from pls.enums.node_type import NodeType
from pls.enums.sort_order import SortOrder
from pls.models.node import Node


console = Console()


def sort_key(node: Node) -> str:
    """
    Map a ``Node`` instance to a string that represents it. This string is used
    to sort a list of ``Node`` instances.

    :param node: the node item to reduce to a string
    :return: a string representative of the node
    """

    key = node.name.lstrip(".").lower()
    if not args.no_dirs_first:
        prefix = "0" if node.node_type == NodeType.FOLDER else "1"
        key = f"{prefix}{key}"
    return key


def parse_nodes(node_name: str) -> Node:
    """
    Parse the node name into a ``Node`` instance. Most of the heavy lifting is
    handled in the ``Node`` class definition itself.

    :param node_name: the name of a node inside the working directory
    :return: a ``Node`` instance
    """

    node_path = args.directory.joinpath(node_name)
    return Node(node_name, path=node_path)


def read_input() -> list[Node]:
    """
    Get a list of all directories and files in the given directory.

    :return: the list of directories and files inside the given directory
    """

    all_nodes = os.listdir(args.directory)
    with multiprocessing.Pool() as pool:
        comp_nodes = pool.map(parse_nodes, all_nodes)
    all_nodes = comp_nodes

    if not all_nodes:
        console.print(
            f"There are no files or folders in [bold]{args.directory}[/bold].",
            highlight=False,
            end="",
        )
    else:
        all_nodes.sort(key=sort_key, reverse=args.sort == SortOrder.DESC)

    return all_nodes
