from __future__ import annotations
from sys import platform
if platform != "win32":
    from grp import getgrgid
    from pwd import getpwuid
from itertools import cycle
from pathlib import Path

from pls.args import args
from pls.enums.node_type import NodeType, type_test_map
from pls.enums.unit_system import get_base_and_pad_and_units


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
        "T": "magenta",
        "s": "magenta",
    }
    perm_sets: list[list[str]] = [["-" for _ in range(3)] for _ in range(3)]

    text_rep = format(st_mode, "09b")[-9:]
    for index, (bit, perm) in enumerate(zip(text_rep, cycle(perms))):
        if int(bit):
            perm_sets[int(index / 3)][index % 3] = perm

    if st_mode & 0o4000 == 0o4000:  # setuid
        perm_sets[0][-1] = "s"
    if st_mode & 0o2000 == 0o2000:  # setgid
        perm_sets[1][-1] = "s"
    if st_mode & 0o1000 == 0o1000:  # sticky
        perm_sets[2][-1] = "T"

    return " ".join(
        "".join(
            f"[{color}]{perm}[/]" if (color := color_map.get(perm, "")) else perm
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

    base, pad, units = get_base_and_pad_and_units(args.units)
    for index, unit in reversed(list(enumerate(units))):
        order_of_magnitude = base ** index
        if st_size >= order_of_magnitude:
            magnitude = round(st_size / order_of_magnitude)  # Pop! Pop!
            unit = f"{unit}B".rjust(pad, " ")

            return f"{magnitude}[dim]{unit}[/]"
    return f"{st_size}  [dim]B[/]"

if platform != "win32":
    def get_user(st_uid: int) -> str:
        """
        Get the name of the user that owns the node. This requires a ``passwd``
        lookup for the user ID found in the node stats.

        :param st_uid: the user ID mapped to the owner of the node
        :return: the name of the user who owns the node
        """

        return getpwuid(st_uid).pw_name


    def get_group(st_gid: int) -> str:
        """
        Get the name of the group that owns the node. This requires a group database
        lookup for the group ID found in the node stats.

        :param st_gid: the group ID mapped to the owner of the node
        :return: the name of the group that owns the node
        """

        return getgrgid(st_gid).gr_name


def get_node_type(path: Path) -> NodeType:
    """
    Get the ``NodeType`` that corresponds to the given mode. This function uses
    functions defined in the ``stat`` module to identify the type.

    :param path: the path to the node
    :return: the right ``NodeType`` enum based on the mode
    """

    for node_type, node_type_test in type_test_map.items():
        if getattr(path, node_type_test)():
            return node_type
    else:
        raise ValueError("Could not determine type of the node.")
