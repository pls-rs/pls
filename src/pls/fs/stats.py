from __future__ import annotations

import datetime
from itertools import cycle
from typing import Optional

from pls.args import args
from pls.enums.unit_system import UnitSystem, get_base_and_pad_and_units
from pls.state import state


def get_permission_text(st_mode: int) -> str:
    """
    Get the permission text for the node in the form of a triplet of 'rwx'
    strings.

    :param st_mode: the st_mode value of the ``os.stat_result`` instance
    :return: the text to render as the permissions of the node
    """

    perms = ["r", "w", "x"]
    color_map = {
        "r": "yellow",
        "w": "red",
        "x": "green",
        "t": "magenta",
        "s": "magenta",
    }
    perm_sets: list[list[str]] = [["-" for _ in range(3)] for _ in range(3)]

    text_rep = format(st_mode, "09b")[-9:]
    for index, (bit, perm) in enumerate(zip(text_rep, cycle(perms))):
        if int(bit):
            perm_sets[int(index / 3)][index % 3] = perm

    if st_mode & 0o4000 == 0o4000:  # setuid
        perm_sets[0][-1] = "s" if perm_sets[0][-1] == "x" else "S"
    if st_mode & 0o2000 == 0o2000:  # setgid
        perm_sets[1][-1] = "s" if perm_sets[1][-1] == "x" else "S"
    if st_mode & 0o1000 == 0o1000:  # sticky
        perm_sets[2][-1] = "t" if perm_sets[2][-1] == "x" else "T"

    return " ".join(
        "".join(
            f"[{color}]{perm}[/]"
            if (color := color_map.get(perm.lower(), ""))
            else perm
            for perm in perm_set
        )
        for perm_set in perm_sets
    )


def get_size(st_size: int) -> str:
    """
    Get the human-readable size of the node in the form of a number followed by
    a compound unit of a byte.

    :param st_size: the size of the node in bytes
    :return: the size of the node as a human-readable value
    """

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


def get_formatted_user(st_uid: int) -> Optional[str]:
    """
    Get the name of the user that owns the node. This requires a ``passwd``
    lookup for the user ID found in the node stats.

    :param st_uid: the user ID mapped to the owner of the node
    :return: the name of the user who owns the node
    """

    try:
        from pwd import getpwuid

        pw_name = getpwuid(st_uid).pw_name
        if pw_name != state.username:
            pw_name = f"[dim]{pw_name}[/]"
        return pw_name
    except ModuleNotFoundError:  # on non-POSIX systems like Windows
        return None


def get_formatted_group(st_gid: int) -> Optional[str]:
    """
    Get the name of the group that owns the node. This requires a group database
    lookup for the group ID found in the node stats.

    :param st_gid: the group ID mapped to the owner of the node
    :return: the name of the group that owns the node
    """

    try:
        from grp import getgrgid

        gr_name = getgrgid(st_gid).gr_name
        if gr_name not in state.groups:
            gr_name = f"[dim]{gr_name}[/]"
        return gr_name
    except ModuleNotFoundError:  # on non-POSIX systems like Windows
        return None


def get_formatted_time(st_time: int) -> str:
    """
    Get the given UNIX timestamp as a formatted human/machine-readable date time
    value. The formatting can be controlled via CLI arguments.

    :param st_time: the UNIX timestamp for creation, modification or access
    :return: the readable date time value for the timestamp
    """

    dt = datetime.datetime.fromtimestamp(st_time)
    fmt = args.time_fmt
    return dt.strftime(fmt)
