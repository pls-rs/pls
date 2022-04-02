from pls.parser.args import dev, filter, info, meta, pos, pres, sort
from pls.parser.base import PlsFormatter, PlsParser


def _get_core_parser() -> PlsParser:
    """
    Get an empty argument parser for ``pls``. This parser has no arguments or argument
    groups associated with it, yet.

    :return: the standard argument parser
    """

    return PlsParser(
        prog="pls",
        description=(
            """
            [red bold]`pls`[/] is a prettier and powerful `ls` for the pros.

            You can read the docs at https://dhruvkb.github.io/pls and
            obtain the source code at https://github.com/dhruvkb/pls.
            """
        ),
        formatter_class=PlsFormatter,
        add_help=False,  # added via the 'meta' group later
    )


def get_parser() -> PlsParser:
    """
    Get the parser with all arguments configured on it.

    :return: the complete ``pls`` argument parser
    """

    core_parser = _get_core_parser()

    arg_modules = [pos, meta, pres, info, sort, filter, dev]
    for arg_module in arg_modules:
        adder = getattr(arg_module, "add_args")
        adder(core_parser)

    return core_parser


parser = get_parser()
"""the standard CLI argument parser for ``pls``"""
