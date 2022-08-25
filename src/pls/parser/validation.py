import argparse
import logging

from pls.output.columns.detail_columns import detail_column_specs


logger = logging.getLogger(__name__)


def validate_args(args: argparse.Namespace):
    """
    Validate arguments in the given namespace. This function mutates the properties in
    the namespace as a side effect and therefore does not return any value.

    :param args: the arguments to validate
    """

    if args.details and args.multi_cols:
        logger.info("Cannot have multiple columns in detailed view.")
        args.multi_cols = False

    if args.multi_cols and args.collapse != 0:
        logger.info("Cannot collapse in multiple columns.")
        args.collapse = 0

    if "std" in args.details:
        args.details.remove("std")
        std_fields = ["type", "perms", "user", "group"]
        args.details.extend(std_fields)

    if "all" in args.details:
        args.details.remove("all")
        all_fields = list(detail_column_specs.keys())
        args.details.extend(all_fields)
