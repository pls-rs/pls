import argparse
import os
from pathlib import Path
from typing import Optional

from pls import __version__
from pls.enums.icon_type import IconType
from pls.enums.sort_order import SortOrder
from pls.enums.unit_system import UnitSystem
from pls.exceptions import ExecException


parser = argparse.ArgumentParser(
    prog="pls",
    description=(
        "`pls` is a better `ls` for developers. "
        "See https://github.com/dhruvkb/pls for more information."
    ),
)

########################
# Positional arguments #
########################


def directory(path_str: str) -> Path:
    """
    Parse the given path into a ``Path`` instance. The path is considered valid
    if it points to an existing directory.

    :param path_str: the path supplied as a CLI argument
    :return: the ``Path`` instance wrapping the supplied path
    :raise: ``ExecException``, if the path is invalid
    """

    path = Path(path_str).resolve()
    if not os.path.isdir(path):
        raise ExecException("`directory` must be a path to a directory")
    return path


parser.add_argument(
    "directory",
    type=directory,
    nargs="?",  # makes the `directory` arg optional
    default=os.getcwd(),
    help="the directory whose contents are to be listed",
)

######################
# Optional arguments #
######################

parser.add_argument(
    *["-v", "--version"],
    action="version",
    version=f"%(prog)s {__version__}",
    help="show the version of the codebase",
)


def file(path_str: str) -> Optional[Path]:
    """
    Parse the given path into a ``Path`` instance. The path is considered valid
    if nothing exists there or if it points to a file.

    :param path_str: the path supplied as a CLI argument
    :return: the ``Path`` instance wrapping the supplied path if it is valid,
        ``None`` otherwise
    """

    path = Path(path_str).resolve()
    if not path.exists() or (path.exists() and path.is_file()):
        return path
    else:
        return None


parser.add_argument(
    *["-e", "--export"],
    type=file,
    default=None,
    help="the path to the file where to write the exported HTML",
)

parser.add_argument(
    *["-i", "--icon"],
    type=IconType,
    choices=list(IconType),
    default=IconType.NERD,
    help="the type of icons to show with the files",
)
parser.add_argument(
    *["-s", "--sort"],
    type=SortOrder,
    choices=list(SortOrder),
    default=SortOrder.ASC,
    help="the direction in which to sort the files and directories",
)
parser.add_argument(
    *["-d", "--depth"],
    type=int,
    default=4,
    help="the max depth upto which to look for a `.pls.yml` file",
)
parser.add_argument(
    *["-u", "--units"],
    type=UnitSystem,
    choices=list(UnitSystem),
    default=UnitSystem.BINARY,
    help="the units to use when listing the size of files",
)

parser.add_argument(
    "--details",
    action="store_true",
    help="show details such as permissions, owner and size",
)
parser.add_argument(
    "--all",
    action="store_true",
    help="show all files, including those that would otherwise be hidden",
)

parser.add_argument(
    "--no-dirs",
    action="store_true",
    help="hide directories in the output",
)
parser.add_argument(
    "--no-files",
    action="store_true",
    help="hide files in the output",
)
parser.add_argument(
    "--no-align",
    action="store_true",
    help="turn off character alignment for leading dots",
)
parser.add_argument(
    "--no-dirs-first",
    action="store_true",
    help="mix directories and files, sorting them together",
)


args = parser.parse_args()
"""the CLI arguments parsed by ``argparse``"""

__all__ = ["args"]
