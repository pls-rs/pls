from __future__ import annotations

import datetime
import os
from itertools import cycle
from stat import S_ISDIR
from typing import Literal, Optional

from pls.args import args
from pls.enums.unit_system import UnitSystem, get_base_and_pad_and_units
from pls.state import state


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

        pw_name = getpwuid(stat.st_uid).pw_name
        if pw_name != state.username:
            pw_name = f"[dim]{pw_name}[/]"
        return pw_name
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

        gr_name = getgrgid(stat.st_gid).gr_name
        if gr_name not in state.groups:
            gr_name = f"[dim]{gr_name}[/]"
        return gr_name
    except ModuleNotFoundError:  # on non-POSIX systems like Windows
        return None


def get_formatted_size(stat: os.stat_result) -> str:
    """
    Get the human-readable size of the node in the form of a number followed by
    a compound unit of a byte. Uses ``st_size`` from the stat results.

    :param stat: the stat results of the node
    :param is_dir: whether the node is a directory
    :return: the size of the node as a human-readable value
    """

    st_size = stat.st_size

    if S_ISDIR(stat.st_mode):
        return "[dim]-[/dim]"

    if args.units == UnitSystem.NONE:
        return f"{st_size}[dim]B[/]"

    base, pad, units = get_base_and_pad_and_units(args.units)
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
    fmt = args.time_fmt
    return dt.strftime(fmt)
