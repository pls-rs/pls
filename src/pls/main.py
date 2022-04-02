#!/usr/bin/env python3
import logging
import os

from pls.config import constants, icons, prefs, specs
from pls.config.files import find_configs
from pls.data.utils import internal_yml_path
from pls.globals import args, state
from pls.log.config import configure_log_level
from pls.parser.parser import parser
from pls.parser.validation import validate_args


logger = logging.getLogger(__name__)


def init(argv=None):
    """
    Initialise module variables.

    :param argv: the argument vector to parse, use ``None`` to read ``sys.argv``
    """

    configure_log_level()

    logger.info("Parsing internal prefs")
    prefs.internal_prefs = prefs.get_prefs([internal_yml_path("prefs.yml")])
    logger.debug(f"Internal preferences: {prefs.internal_prefs}")

    logger.info("Parsing CLI arguments")
    cli_prefs = parser.parse_args(argv)
    logger.debug(f"CLI arguments: {cli_prefs}")

    args.args.node = cli_prefs.node

    state.state = state_obj = state.State()

    state_obj.setup_home()
    state_obj.setup_user_groups()
    state_obj.setup_git(cli_prefs.node)

    conf_files = find_configs(cli_prefs.node)
    logger.debug(f"Config files read: {conf_files}")

    logger.info("Reading config files")
    prefs.prefs = prefs.get_prefs(conf_files)
    logger.debug(f"Config preferences: {prefs.prefs}")

    args.args.update(prefs.internal_prefs)
    args.args.update(prefs.prefs)
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

    from pls.globals import console

    console.console = console.get_console()  # depends on args


def main() -> None:
    """
    Represents the starting point of the application. This function:

    - accepts no inputs: options are read from CLI arguments using ``argparse``
    - returns no outputs: output is written to ``STDOUT`` using ``rich``
    """

    init(None)  # ``None`` makes ``argparse`` read real CLI args from ``sys.argv``

    from pls.fs.list import read_input
    from pls.output.columns import ColumnsPrinter
    from pls.output.table import TablePrinter

    node_map, node_list = read_input()

    if not node_list:
        return

    for node in node_list:
        node.match_specs(specs.node_specs)
        if args.args.collapse:
            node.find_main(node_map)
    if args.args.collapse:
        for node in node_list:
            if node.is_sub:
                continue
            node.set_sub_pre_shapes()

    PrinterClass = ColumnsPrinter if args.args.multi_cols else TablePrinter
    printer = PrinterClass(node_list)
    printer.print()


def dev() -> None:
    os.environ.setdefault("PLS_LOG_LEVEL", "DEBUG")  # Show detailed logs

    main()


if __name__ == "__main__":
    main()
