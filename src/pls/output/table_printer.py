from __future__ import annotations

from typing import Callable

from rich.table import Table

from pls.globals import args
from pls.models.column_spec import ColumnSpec
from pls.models.node import Node
from pls.output.columns.all_columns import column_groups, column_specs
from pls.output.printers import BasePrinter


class TablePrinter(BasePrinter):
    """
    Render nodes in a table.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.cols: list[ColumnSpec] = self._get_cols()
        self.table = self._get_table()

    @staticmethod
    def _filter_groups(
        all_col_groups: list[list[str]],
        extra_cond: Callable[[ColumnSpec], bool] = lambda _: True,
    ) -> list[list[ColumnSpec]]:
        """
        Given a group of column groups, filter out all unavailable columns.

        :param all_col_groups: the group of column groups to filter
        :return: the filtered list of column groups
        """

        return [
            [
                spec
                for col in col_group
                if (spec := column_specs[col]).is_available and extra_cond(spec)
            ]
            for col_group in all_col_groups
        ]

    @staticmethod
    def _get_cols() -> list[ColumnSpec]:
        """
        Get the list of columns to show.

        :return: the list of column keys
        """

        detail_col_groups, required_col_groups = column_groups
        selected_col_groups = []
        if args.args.details:
            selected_col_groups.extend(
                TablePrinter._filter_groups(
                    detail_col_groups, lambda spec: spec.key in args.args.details
                )
            )
        selected_col_groups.extend(TablePrinter._filter_groups(required_col_groups))

        flattened_cols = []
        for index, col_group in enumerate(selected_col_groups):
            # Skip groups with zero chosen columns.
            if len(col_group) == 0:
                continue

            # Don't add spacer after last group.
            if index != len(selected_col_groups) - 1:
                col_group.append(column_specs["spacer"])
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
            header_style="underline bold",
        )
        for col in self.cols:
            table.add_column(col.name, **col.attrs)
        return table

    def tabulate_node(self, node: Node):
        """
        Add all cells for the given node to the table. If a node has sub-nodes this will
        recursively tabulate them as well.

        :param node: the node to insert into the table
        """

        data = node.table_row
        if data is not None:
            cells = [data.get(col.key, col.value or "") for col in self.cols]
            self.table.add_row(*cells)
            for sub_node in node.children:
                if isinstance(sub_node, Node):
                    self.tabulate_node(sub_node)

    def print_output(self):
        for node in self.all_nodes:
            # Sub-nodes are not tabulated with the rest of the top-level nodes.
            if node.is_sub:
                continue
            self.tabulate_node(node)

        self.console.print(self.table)
