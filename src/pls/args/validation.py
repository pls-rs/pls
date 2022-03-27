import argparse
import logging


logger = logging.getLogger(__name__)


def validate_args(args: argparse.Namespace):
    """
    Validate arguments in the given namespace. This function mutates the properties in
    the namespace as a side effect and therefore does not return any value.

    :param args: the arguments to validate
    """

    if args.details and args.multi_cols:
        logger.warning("Cannot have multiple columns in detailed view.")
        args.multi_cols = False

    if args.multi_cols and args.collapse != 0:
        logger.warning("Cannot collapse in multiple columns.")
        args.collapse = 0
