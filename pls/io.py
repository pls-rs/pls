from __future__ import annotations

import os

from rich.console import Console
from rich.table import Table

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

    return node.name.lstrip(".").lower()


def read_input() -> list[Node]:
    """
    Get a list of all directories and files in the given directory.

    :return: the list of directories and files inside the given directory
    """

    _, dir_names, file_names = next(os.walk(args.directory))

    dirs = []
    if not args.no_dirs:
        dirs = [Node(dir_name, NodeType.FOLDER) for dir_name in dir_names]

    files = []
    if not args.no_files:
        files = [Node(file_name, NodeType.FILE) for file_name in file_names]

    is_reverse = args.sort == SortOrder.DESC
    if not args.no_dirs_first:
        # Sort dirs and files separately and then join
        dirs.sort(key=sort_key, reverse=is_reverse)
        files.sort(key=sort_key, reverse=is_reverse)
    all_nodes = dirs + files
    if args.no_dirs_first:
        # Join dirs and files and then sort together
        all_nodes.sort(key=sort_key, reverse=is_reverse)

    if not all_nodes:
        console.print(
            f"There are no files or folders in [bold]{args.directory}[/bold].",
            highlight=False,
            end="",
        )

    return all_nodes


def write_output(nodes: list[Node]):
    """
    Write the list of all files and directories to the screen.

    :param nodes: the list of all nodes in the given directory
    """

    # Padding set to 2 because most icons are 2 rows wide but occupy only 1 row,
    # thus spilling into one row of the padding.
    table = Table(padding=(0, 2, 0, 0), box=None, show_header=False)

    for node in nodes:
        if not node.is_visible:
            continue
        table.add_row(*node.table_row)

    console.print(table)
