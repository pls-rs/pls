from __future__ import annotations

from rich.table import Table

from pls.enums.icon_type import IconType
from pls.globals import args
from pls.models.node import Node
from pls.output.column_spec import column_groups, column_spec_map
from pls.output.printers import BasePrinter


class TablePrinter(BasePrinter):
    """
    Render nodes in a table.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.cols = self._get_cols()
        self.table = self._get_table()

    @staticmethod
    def _column_chosen(col_name: str) -> bool:
        """
        Determine whether the given column name has been asked for in the details.

        :param col_name: the name of the column to check
        :return: ``True`` if the column is to be shown, ``False`` otherwise
        """

        default_details = [
            "type",
            "perms",
            "user",
            "group",
        ]

        return (
            col_name in args.args.details
            or "all" in args.args.details
            or (col_name in default_details and "def" in args.args.details)
        )

    @staticmethod
    def _get_cols() -> list[str]:
        """
        Get the list of columns to show.

        :return: the list of column keys
        """

        selected_col_groups = []
        if args.args.details:
            for col_group in column_groups:
                filtered_group = [
                    col for col in col_group if TablePrinter._column_chosen(col)
                ]
                selected_col_groups.append(filtered_group)

        name_group = ["name"]
        if args.args.icon != IconType.NONE:
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

    def _get_table(self) -> Table:
        """
        Get a Rich table with pre-configured columns. The attributes of the columns
        are retrieved from ``column_spec`` based on keys from ``get_columns``.

        :return: a Rich table
        """

        table = Table(
            padding=(0, 1, 0, 0),
            box=None,
            show_header=bool(args.args.details),
            header_style="underline",
        )
        for col_key in self.cols:
            col = column_spec_map.get(col_key)
            if col is not None:
                table.add_column(col.get("name", ""), **col.get("attrs", {}))
        return table

    def tabulate_node(self, node: Node):
        """
        Add all cells for the given node to the table. If a node has sub-nodes this will
        recursively tabulate them as well.

        :param node: the node to insert into the table
        """

        data = node.table_row
        if data is not None:
            cells = [data.get(col, "") for col in self.cols]
            self.table.add_row(*cells)
            for sub_node in node.children:
                self.tabulate_node(sub_node)

    def print_output(self):
        for node in self.all_nodes:
            # Sub-nodes are not tabulated with the rest of the top-level nodes.
            if node.is_sub:
                continue
            self.tabulate_node(node)

        self.console.print(self.table)
