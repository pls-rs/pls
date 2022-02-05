from pathlib import Path
from pwd import getpwuid

from pls.args import args
from pls.enums.node_type import NodeType, type_test_map
from pls.enums.unit_system import UnitSystem, get_base_and_units


def get_permission_text(st_mode: int) -> str:
    """
    Get the permission text for the node in the form of a triplet of 'rwx'
    strings.

    :param st_mode: the st_mode value of the ``os.stat_result`` instance
    :return: the text to render as the permissions of the node
    """

    perms = {"r": "yellow", "w": "red", "x": "green"}
    perm_set = [[] for _ in range(3)]

    octal_text = oct(st_mode)[-3:]
    for index, mode in enumerate(octal_text):
        bit = int(mode)
        for perm_index, (perm, color) in enumerate(perms.items()):
            if bit >= (perm_val := 2 ** (len(perms) - perm_index - 1)):
                perm_set[index].append(f"[{color}]{perm}[/]")
                bit -= perm_val
            else:
                perm_set[index].append("-")

    return " ".join("".join(perms) for perms in perm_set)


def get_size(st_size: int) -> str:
    """
    Get the human-readable size of the node in the form of a number followed by
    a compound unit of a byte.

    :param st_size: the size of the node in bytes
    :return: the size of the node as a human-readable value
    """

    base, units = get_base_and_units(args.units)
    for index, unit in reversed(list(enumerate(units))):
        order_of_magnitude = base ** index
        if st_size >= order_of_magnitude:
            magnitude = round(st_size / order_of_magnitude)  # Pop! Pop!

            pad = 2 if args.units == UnitSystem.DECIMAL else 3
            unit = f"{unit}B".rjust(pad, " ")

            return f"{magnitude}[dim]{unit}[/]"
    return f"{st_size}? "


def get_username(st_uid: int) -> str:
    """
    Get the name of the user that owns the node. This requires a ``passwd``
    lookup for the user ID found in the node stats.

    :param st_uid: the user ID mapped to the owner of the node
    :return: the name of the user who owns the node
    """

    return getpwuid(st_uid).pw_name


def get_node_type(path: Path) -> NodeType:
    """
    Get the ``NodeType`` that corresponds to the given mode. This function uses
    functions defined in the ``stat`` module to identify the type.

    :param path :the st_mode value of the ``os.stat_result`` instance
    :return: the right ``NodeType`` enum based on the mode
    """

    for node_type, node_type_test in type_test_map.items():
        if getattr(path, node_type_test)():
            return node_type
    else:
        raise ValueError("Could not determine type of the node.")
