import argparse
import logging
import string
from pathlib import Path
from random import choice
from typing import Optional

logger = logging.getLogger(__name__)


def _file(path_str: str) -> Optional[Path]:
    """
    Parse the given path into a ``Path`` instance. The path is considered valid
    if nothing exists there or if it points to a file.

    :param path_str: the path supplied as a CLI argument
    :return: the ``Path`` instance wrapping the supplied path if it is valid
    :raise: ``ArgException``, if the path is invalid
    """

    path = Path(path_str).resolve()

    if path.exists():
        rand = "".join(choice(string.hexdigits.lower()) for _ in range(8))
        target = f"{path}.{rand}"
        logger.warning(f"Something already exists at {path}, renaming to {target}")
        path.rename(target)

    return path


def add_args(parser: argparse.ArgumentParser):
    """
    Add arguments for ``pls`` development to the given parser.

    :param parser: the parser to which to add the arguments
    """

    dev = parser.add_argument_group(
        title="development",
        description="arguments useful when developing pls",
    )
    dev.add_argument(
        *["-x", "--export"],
        type=_file,
        help="the path to the file where to write the exported HTML",
    )
