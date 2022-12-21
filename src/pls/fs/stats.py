from __future__ import annotations

import datetime
import logging
from functools import lru_cache
from stat import filemode
from typing import Optional

from pls.config import constants
from pls.enums.node_type import NodeType
from pls.enums.unit_system import UnitSystem
from pls.globals import args, state
from pls.models.format_rules import FormatRules

logger = logging.getLogger(__name__)


@lru_cache(maxsize=None)
def get_formatted_links(node_type: NodeType, st_nlink: int) -> str:
    """
    Get the number of hard links pointing to the file. This is usually higher
    than 1 for directories (as all files and folders within in it counted) but
    usually exactly 1 for files.

    :param node_type: the type of the node
    :param st_nlink: the number of links retrieved from the ``stat`` call
    :return: the number of links of a file
    """

    nlink = str(st_nlink)
    if node_type != NodeType.DIR and st_nlink > 1:
        nlink = f"[yellow]{nlink}[/]"
    return nlink


@lru_cache(maxsize=None)
def get_formatted_perms(st_mode: int) -> str:
    """
    Get the permission text for the node in the form of a triplet of 'rwx'
    strings. Uses ``st_mode`` from the stat results.

    :param st_mode: the file mode retrieved from the ``stat`` call
    :return: the text to render as the permissions of the node
    """

    perm = filemode(st_mode)[1:]  # drop the first letter, i.e. type char
    perm = f"{perm[:3]} {perm[3:6]} {perm[6:]}"

    formatted_perm = ""
    for char in perm:
        color = constants.constants.lookup("permission_styles", char, default="dim")
        formatted_perm += f"[{color}]{char}[/]"

    return formatted_perm


@lru_cache(maxsize=None)
def get_formatted_user(st_uid: int) -> Optional[str]:
    """
    Get the name of the user that owns the node. This requires a ``passwd``
    lookup for the user ID found in the node stats. Uses ``st_uid`` from the
    stat results.

    :param st_uid: the user ID retrieved from the ``stat`` call
    :return: the name of the user who owns the node, ``None`` on Windows
    """

    try:
        from pwd import getpwuid

        format_rules = FormatRules()
        try:
            pw_name = getpwuid(st_uid).pw_name
        except KeyError:  # user does not exist anymore
            pw_name = str(st_uid)
            format_rules.append("red")

        if st_uid != state.state.uid:
            format_rules.append("dim")

        return format_rules.format_text(pw_name)
    except ModuleNotFoundError:  # on non-POSIX systems like Windows
        return None


@lru_cache(maxsize=None)
def get_formatted_group(st_gid: int) -> Optional[str]:
    """
    Get the name of the group that owns the node. This requires a group database
    lookup for the group ID found in the node stats. Uses ``st_gid`` from the
    stat results.

    :param st_gid: the group ID retrieved from the ``stat`` call
    :return: the name of the group that owns the node, ``None`` on Windows
    """

    try:
        from grp import getgrgid

        format_rules = FormatRules()
        try:
            gr_name = getgrgid(st_gid).gr_name
        except KeyError:  # group does not exist anymore
            gr_name = str(st_gid)
            format_rules.append("red")

        if st_gid not in state.state.gids:
            format_rules.append("dim")

        return format_rules.format_text(gr_name)
    except ModuleNotFoundError:  # on non-POSIX systems like Windows
        return None


def get_formatted_size(node_type: NodeType, st_size: int) -> str:
    """
    Get the human-readable size of the node in the form of a number followed by
    a compound unit of a byte. Uses ``st_size`` from the stat results.

    :param node_type: the type of the node
    :param st_size: the size of the file retrieved from the ``stat`` call
    :return: the size of the node as a human-readable value
    """

    if node_type == NodeType.DIR:
        return "[dim]-[/dim]"

    if args.args.units == UnitSystem.NONE:
        return f"{st_size}[dim]B[/]"

    unit_system = args.args.units or UnitSystem.BINARY
    base, pad, units = unit_system.base_pad_units
    logger.debug(f"Base: {base}")
    logger.debug(f"Pad: {pad}")
    logger.debug(f"Units: {units}")
    for index, unit in reversed(list(enumerate(units))):
        order_of_magnitude = base**index
        if st_size >= order_of_magnitude:
            magnitude = round(st_size / order_of_magnitude)  # Pop! Pop!
            unit = f"{unit}B".rjust(pad, " ")

            return f"{magnitude}[dim]{unit}[/]"
    return f"{st_size}  [dim]B[/]"


def get_formatted_time(st_time: Optional[int]) -> str:
    """
    Get the given UNIX timestamp as a formatted human/machine-readable date time
    value. The formatting can be controlled via CLI arguments. The name of the
    stat result attribute to use is passed via argument.

    :param st_time: the timestamp retrieved from the ``stat`` call
    :return: the readable date time value for the timestamp
    """

    if st_time is None:
        return ""

    dt = datetime.datetime.fromtimestamp(st_time)
    fmt = args.args.time_fmt
    return dt.strftime(fmt)
