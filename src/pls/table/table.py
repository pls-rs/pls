from __future__ import annotations

import textwrap

from rich.console import Console
from rich.table import Table

from pls.args import args
from pls.data.solarized import solarized_theme
from pls.enums.icon_type import IconType
from pls.models.node import Node
from pls.table.column_spec import column_groups, column_spec_map


console = Console(record=(args.export is not None))


def column_chosen(col_name):
    return col_name in args.details or "+" in args.details


def get_columns() -> list[str]:
    """
    Get the list of columns to show.

    :return: the list of column keys
    """

    selected_col_groups = []
    if args.details:
        for col_group in column_groups:
            filtered_group = [col for col in col_group if column_chosen(col)]
            selected_col_groups.append(filtered_group)

    name_group = ["name"]
    if args.icon != IconType.NONE:
        name_group.insert(0, "icon")
    selected_col_groups.append(name_group)

    flattened_cols = []
    for index, col_group in enumerate(selected_col_groups):
        if len(col_group) == 0:  # skip groups with zero chosen columns
            continue
        if index != len(selected_col_groups) - 1:  # no spacer after last group
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
        show_header=args.details is not None,
        header_style="underline",
    )
    for col_key in get_columns():
        col = column_spec_map.get(col_key)
        if col is not None:
            table.add_column(col.get("name", ""), **col.get("attrs", {}))
    return table


def write_output(all_nodes: list[Node]):
    """
    Write the list of directories and files to the screen as a table.

    :param all_nodes: the list of directories and files
    """

    table = get_table()

    for node in all_nodes:
        data = node.table_row
        if data is not None:
            cells = [data.get(col, "") for col in get_columns()]
            table.add_row(*cells)

    console.print(table)

    if args.export:
        html_body = textwrap.dedent(
            """
            <div
                style="background-color: {background}; color: {foreground};"
                class="language-">
              <pre style="color: inherit;"><code style="color: inherit;">{code}</code></pre>
            </div>
            """  # noqa: E501
        )
        with args.export.open("w", encoding="utf-8") as out_file:
            out_file.write(
                console.export_html(
                    theme=solarized_theme, code_format=html_body, inline_styles=True
                )
            )
        print("Output written to file.")
