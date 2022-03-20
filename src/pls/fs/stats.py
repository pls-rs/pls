from __future__ import annotations

import datetime
import os
from itertools import cycle
from stat import S_ISDIR
from typing import Literal, Optional

from pls.enums.unit_system import UnitSystem, get_base_and_pad_and_units
from pls.globals import state


def _get_format_pair(rules: list[str]) -> tuple[str, str]:
    """
    Get the format pair to mark up the console output for the given rules.

    :param rules: the set of formatting directives to apply on the text
    :return: the pair of format strings to place on either side of the text
    """

    if not rules:
        return "", ""
    left = " ".join(rules)
    return f"[{left}]", "[/]"


def get_formatted_links(stat: os.stat_result) -> str:
    """
    Get the number of hard links pointing to the file. This is usually higher
    than 1 for directories (as all files and folders within in it counted) but
    usually exactly 1 for files.

    :param stat: the stat results of the node
    :return: the number of links of a file
    """

    st_nlink = stat.st_nlink

    nlink = str(st_nlink)
    if not S_ISDIR(stat.st_mode) and st_nlink > 1:
        nlink = f"[yellow]{nlink}[/]"
    return nlink


def get_formatted_perms(stat: os.stat_result) -> str:
    """
    Get the permission text for the node in the form of a triplet of 'rwx'
    strings. Uses ``st_mode`` from the stat results.

    :param stat: the stat results of the node
    :return: the text to render as the permissions of the node
    """

    st_mode = stat.st_mode

    perms = ["r", "w", "x"]
    specials = ["s", "s", "t"]
    color_map = {
        "r": "yellow",
        "w": "red",
        "x": "green",
        "t": "magenta",
        "s": "magenta",
    }
    perm_sets: list[list[str]] = [["-" for _ in range(3)] for _ in range(3)]

    text_rep = format(st_mode, "012b")[-12:]

    for index, (bit, perm) in enumerate(zip(text_rep[3:], cycle(perms))):
        if int(bit):
            perm_sets[int(index / 3)][index % 3] = perm
    for index, (bit, spl) in enumerate(zip(text_rep[:3], specials)):
        if int(bit):
            perm_sets[index][2] = spl if perm_sets[index][2] == "x" else spl.upper()

    return " ".join(
        "".join(
            f"[{color}]{perm}[/]"
            if (color := color_map.get(perm.lower(), ""))
            else perm
            for perm in perm_set
        )
        for perm_set in perm_sets
    )


def get_formatted_user(stat: os.stat_result) -> Optional[str]:
    """
    Get the name of the user that owns the node. This requires a ``passwd``
    lookup for the user ID found in the node stats. Uses ``st_uid`` from the
    stat results.

    :param stat: the stat results of the node
    :return: the name of the user who owns the node, ``None`` on Windows
    """

    try:
        from pwd import getpwuid

        uid = stat.st_uid
        format_rules = []
        try:
            pw_name = getpwuid(stat.st_uid).pw_name
        except KeyError:  # user does not exist anymore
            pw_name = str(uid)
            format_rules.append("red")

        if uid != state.state.uid:
            format_rules.append("dim")

        left, right = _get_format_pair(format_rules)
        return f"{left}{pw_name}{right}"
    except ModuleNotFoundError:  # on non-POSIX systems like Windows
        return None


def get_formatted_group(stat: os.stat_result) -> Optional[str]:
    """
    Get the name of the group that owns the node. This requires a group database
    lookup for the group ID found in the node stats. Uses ``st_gid`` from the
    stat results.

    :param stat: the stat results of the node
    :return: the name of the group that owns the node, ``None`` on Windows
    """

    try:
        from grp import getgrgid

        gid = stat.st_gid
        format_rules = []
        try:
            gr_name = getgrgid(stat.st_gid).gr_name
        except KeyError:  # group does not exist anymore
            gr_name = str(gid)
            format_rules.append("red")

        if gid not in state.state.gids:
            format_rules.append("dim")

        left, right = _get_format_pair(format_rules)
        return f"{left}{gr_name}{right}"
    except ModuleNotFoundError:  # on non-POSIX systems like Windows
        return None


def get_formatted_size(stat: os.stat_result) -> str:
    """
    Get the human-readable size of the node in the form of a number followed by
    a compound unit of a byte. Uses ``st_size`` from the stat results.

    :param stat: the stat results of the node
    :return: the size of the node as a human-readable value
    """

    st_size = stat.st_size

    if S_ISDIR(stat.st_mode):
        return "[dim]-[/dim]"

    if state.state.units == UnitSystem.NONE:
        return f"{st_size}[dim]B[/]"

    base, pad, units = get_base_and_pad_and_units(state.state.units)
    for index, unit in reversed(list(enumerate(units))):
        order_of_magnitude = base ** index
        if st_size >= order_of_magnitude:
            magnitude = round(st_size / order_of_magnitude)  # Pop! Pop!
            unit = f"{unit}B".rjust(pad, " ")

            return f"{magnitude}[dim]{unit}[/]"
    return f"{st_size}  [dim]B[/]"


def get_formatted_time(
    stat: os.stat_result, attr_name: Literal["st_ctime", "st_mtime", "st_atime"]
) -> str:
    """
    Get the given UNIX timestamp as a formatted human/machine-readable date time
    value. The formatting can be controlled via CLI arguments. The name of the
    stat result attribute to use is passed via argument.

    :param stat: the stat results of the node
    :param attr_name: the name of the UNIX timestamp attribute to use
    :return: the readable date time value for the timestamp
    """

    st_time = getattr(stat, attr_name)

    dt = datetime.datetime.fromtimestamp(st_time)
    fmt = state.state.time_fmt
    return dt.strftime(fmt)
