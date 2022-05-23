from __future__ import annotations

import logging
import textwrap
from pathlib import Path

from pls.globals import args, console
from pls.models.node import Node
from pls.output.solarized import solarized_theme


logger = logging.getLogger(__name__)


class BasePrinter:
    """
    Defines the blueprint of a printer that renders output to the screen.
    """

    def __init__(self, cwd: Path, all_nodes: list[Node]):
        self.console = console.console
        self.cwd = cwd
        self.all_nodes = all_nodes

    def print(self, show_header: bool = False):
        """
        Perform the print and post-print actions.
        """

        if show_header:
            self.print_header()
        self.print_output()
        self.post_print()

    def print_header(self):
        """
        Print the name of the node to the output.
        """

        self.console.print(f"{self.cwd.name}:")

    def print_output(self):
        """
        Print the nodes to the output. This function must be implemented in all
        subclasses.
        """

        raise NotImplementedError(
            "Use a concrete printer class, such as `TablePrinter` or `ColumnsPrinter`."
        )

    def _write_to_file(self):
        """
        Write the contents of the console to an HTML file. This HTML snippet can then be
        used as example in the documentation.
        """

        html_body = textwrap.dedent(
            """
            <div
                style="background-color: {background}; color: {foreground};"
                class="language-">
              <pre style="color: inherit;"><code style="color: inherit;">{code}</code></pre>
            </div>
            """  # noqa: E501
        )
        with args.args.export.open("w", encoding="utf-8") as out_file:
            content = self.console.export_html(
                theme=solarized_theme,
                code_format=html_body,
                inline_styles=True,
            )
            out_file.write(content)
            logger.info(f"Output written to file {args.args.export}.")

    def post_print(self):
        """
        Perform actions after printing to the console, for example, exporting the output
        to a file if the `--export`/`-x` flag was passed.
        """

        if args.args.export:
            self._write_to_file()
