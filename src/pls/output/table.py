from __future__ import annotations

import textwrap

from rich.console import Console
from rich.table import Table

from pls import globals
from pls.enums.icon_type import IconType
from pls.models.node import Node
from pls.output.column_spec import column_groups, column_spec_map
from pls.output.solarized import solarized_theme


console = Console(record=(globals.state.export is not None))


def column_chosen(col_name: str) -> bool:
    """
    Determine whether the given column name has been asked for in the details.

    :param col_name: the name of the column to check
    :return: ``True`` if the column is to be shown, ``False`` otherwise
    """

    return col_name in globals.state.details or "+" in globals.state.details


def get_columns() -> list[str]:
    """
    Get the list of columns to show.

    :return: the list of column keys
    """

    selected_col_groups = []
    if globals.state.details:
        for col_group in column_groups:
            filtered_group = [col for col in col_group if column_chosen(col)]
            selected_col_groups.append(filtered_group)

    name_group = ["name"]
    if globals.state.icon != IconType.NONE:
        name_group.insert(0, "icon")
    selected_col_groups.append(name_group)

    flattened_cols = []
    for index, col_group in enumerate(selected_col_groups):
        # Skip groups with zero chosen columns.
        if len(col_group) == 0:
            continue

        # Don't add spacer after last group.
        if index != len(selected_col_groups) - 1:
            col_group.append("spacer")
        flattened_cols.extend(col_group)

    return flattened_cols


def get_table() -> Table:
    """
    Get a Rich table with pre-configured columns. The attributes of the columns
    are retrieved from ``column_spec`` based on keys from ``get_columns``.

    :return: a Rich table
    """

    table = Table(
        padding=(0, 1, 0, 0),
        box=None,
        show_header=globals.state.details is not None,
        header_style="underline",
    )
    for col_key in get_columns():
        col = column_spec_map.get(col_key)
        if col is not None:
            table.add_column(col.get("name", ""), **col.get("attrs", {}))
    return table


def tabulate_node(table: Table, node: Node):
    """
    Add all cells for the given node to the given table. If a node has sub-nodes
    this will recursively tabulate them as well.

    :param table: the table in which to print the node
    :param node: the node to insert into the table
    """

    data = node.table_row
    if data is not None:
        cells = [data.get(col, "") for col in get_columns()]
        table.add_row(*cells)
        for sub_node in node.children:
            tabulate_node(table, sub_node)


def write_output(all_nodes: list[Node]):
    """
    Write the list of directories and files to the screen as a table. If the
    ``--export`` flag is set, the output is also written as HTML markup to the
    given file.

    :param all_nodes: the list of all directories and files
    """

    table = get_table()

    for node in all_nodes:
        # Sub-nodes are not tabulated with the rest of the top-level nodes.
        if node.is_sub:
            continue
        tabulate_node(table, node)

    console.print(table)

    if globals.state.export:
        html_body = textwrap.dedent(
            """
            <div
                style="background-color: {background}; color: {foreground};"
                class="language-">
              <pre style="color: inherit;"><code style="color: inherit;">{code}</code></pre>
            </div>
            """  # noqa: E501
        )
        with globals.state.export.open("w", encoding="utf-8") as out_file:
            content = console.export_html(
                theme=solarized_theme,
                code_format=html_body,
                inline_styles=True,
            )
            out_file.write(content)
            print("Output written to file.")
