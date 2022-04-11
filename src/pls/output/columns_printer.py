from __future__ import annotations

from typing import Optional

from rich.columns import Columns

from pls.enums.icon_type import IconType
from pls.globals import args
from pls.models.node import Node
from pls.output.printers import BasePrinter
from pls.utils.strip_fmt import strip_formatting


class ColumnsPrinter(BasePrinter):
    """
    Render nodes in columns.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.cols = self._get_cols()
        self.columns = self._get_columns()

    @staticmethod
    def _get_cols() -> list[str]:
        """
        Get the list of columns to show.

        :return: the list of column keys
        """

        columns = ["name"]
        if args.args.icon != IconType.NONE:
            columns.insert(0, "icon")
        return columns

    @staticmethod
    def _get_columns() -> Columns:
        """
        Get a Rich column layout with pre-configured settings. The columns are set to a
        fixed width later, determined by the name of the longest node.

        :return: a Rich column layout
        """

        return Columns(
            expand=True,
            column_first=True,
        )

    def render_node(self, node: Node) -> Optional[int]:
        """
        Add the name of the given node to the given column layout.

        :param node: the node to print in the column layout
        """

        data = node.table_row
        if data is None:
            return None

        name = " ".join([(data.get(col) or " ") for col in self.cols])
        self.columns.add_renderable(name)

        width = len(strip_formatting(name))
        return width

    def print_output(self):
        max_width = 0
        for node in self.all_nodes:
            width = self.render_node(node)
            if width is not None and width > max_width:
                max_width = width

        self.columns.width = max_width + 2  # safety margin
        self.console.print(self.columns)
