#!/usr/bin/env python3
import argparse
import logging
import os
from pathlib import Path
from typing import Type

from pls.config import constants, icons, prefs, specs
from pls.config.files import find_configs
from pls.data.utils import internal_yml_path
from pls.fs.list import read_input
from pls.globals import args, console, state
from pls.log.config import configure_log_level
from pls.output.printers import BasePrinter
from pls.parser.parser import parser
from pls.parser.validation import validate_args


logger = logging.getLogger(__name__)


def general_init(argv=None) -> argparse.Namespace:
    """
    Initialise the global state that does not depend on the working node.

    :param argv: the argument vector to parse, use ``None`` to read ``sys.argv``
    """

    configure_log_level()

    logger.info("Parsing internal prefs")
    prefs.internal_prefs = prefs.get_prefs([internal_yml_path("prefs.yml")])
    logger.debug(f"Internal preferences: {prefs.internal_prefs}")

    logger.info("Parsing CLI arguments")
    cli_prefs = parser.parse_args(argv)
    logger.debug(f"CLI arguments: {cli_prefs}")

    for attr in ["nodes", "export"]:
        setattr(args.args, attr, getattr(cli_prefs, attr))

    # Console
    console.console = console.get_console()

    # State
    state.state = state_obj = state.State()

    state_obj.setup_user_conf()
    state_obj.setup_home()
    state_obj.setup_user_groups()

    return cli_prefs


def node_specific_init(node: Path, cli_prefs: argparse.Namespace):
    """
    Initialise the global state that depends on the working node.

    :param node: the node being described in the current iteration
    :param cli_prefs: the arguments parsed from the CLI
    """

    state.state.setup_git(node)

    conf_files = find_configs(node)
    logger.debug(f"Config files read: {conf_files}")

    logger.info("Reading config files")
    prefs.config_prefs = prefs.get_prefs(conf_files)
    logger.debug(f"Config preferences: {prefs.config_prefs}")

    args.args.update(prefs.internal_prefs)
    args.args.update(prefs.config_prefs)
    args.args.update(cli_prefs)
    validate_args(args.args)

    logger.info("Reading icons")
    icons.nerd_icons, icons.emoji_icons = icons.get_icons(
        [
            *conf_files,
            internal_yml_path("nerd_icons.yml"),
            internal_yml_path("emoji_icons.yml"),
        ]
    )
    logger.debug(f"Nerd icons count: {len(icons.nerd_icons)}")
    logger.debug(f"Emoji icons count: {len(icons.emoji_icons)}")

    logger.info("Reading constants")
    constants.constants = constants.get_constants(
        [
            *conf_files,
            internal_yml_path("constants.yml"),
        ]
    )
    logger.debug(f"Constants count: {len(constants.constants)}")

    logger.info("Reading node specs")
    specs.node_specs = specs.get_specs(
        [
            *conf_files,
            internal_yml_path("node_specs.yml"),
        ]
    )
    logger.debug(f"Node specs count: {len(specs.node_specs)}")


def treerender(node, child_list, show_header: bool):
    """
    Required subprocess to populate and render a Live Tree output
    """

    from rich.live import Live

    from pls.output.table_printer import TablePrinter

    printer = TablePrinter(node, child_list)

    with Live(
        printer.table,
        vertical_overflow="visible",
        refresh_per_second=20,
    ):

        def populate_callback(_):
            printer.tabulate_node(_)

        for child in child_list:
            child.populate_tree(specs.node_specs, populate_callback=populate_callback)


def main_unit(node: Path, show_header: bool = False):
    """
    This function is the job of main, extracted outside the loop.

    :param node: the node being described in this iteration
    :param show_header: whether to show the name of the working node before the output
    """

    child_map, child_list = read_input(node)

    # If there are no children in node, move on.
    if not child_list:
        return

    if args.args.tree:
        return treerender(node=node, child_list=child_list, show_header=show_header)

    for child in child_list:
        child.spec_comp.match(specs.node_specs)

        if args.args.collapse:
            child.collapse_comp.find_main(child_map)
    if args.args.collapse:
        for child in child_list:
            if child.is_sub:
                continue
            child.set_sub_pre_shapes()

    PrinterClass: Type[BasePrinter]
    if args.args.multi_cols:
        from pls.output.columns_printer import ColumnsPrinter

        PrinterClass = ColumnsPrinter
    else:
        from pls.output.table_printer import TablePrinter

        PrinterClass = TablePrinter

    printer = PrinterClass(node, child_list)
    printer.print(show_header)


def main():
    """
    Represents the starting point of the application. This function:

    - accepts no inputs: options are read from CLI arguments using ``argparse``
    - returns no outputs: output is written to ``STDOUT`` using ``rich``
    """

    cli_prefs = general_init(None)  # Read real CLI args from ``sys.argv``

    node_counts = len(args.args.nodes)
    show_header = node_counts > 1

    for index, node in enumerate(args.args.nodes):
        node_specific_init(node, cli_prefs)

        main_unit(node, show_header)

        if index != node_counts - 1:
            console.console.print()  # Separate outputs using blank lines.


def dev():
    os.environ.setdefault("PLS_LOG_LEVEL", "DEBUG")  # Show detailed logs

    main()


if __name__ == "__main__":
    main()
