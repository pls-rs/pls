from enum import auto

from pls.enums.base import AutoEnum


class UnitSystem(AutoEnum):
    """
    Bytes can be converted to higher units in two ways:

    - the decimal system where powers of 10 (factor: 10^3) are used
    - the binary system where powers of 2 (factor: 2^10) are used

    This enum lists these possibilities.
    """

    BINARY = auto()
    DECIMAL = auto()


def get_base_and_units(us: UnitSystem) -> [int, list[str]]:
    """
    Get the base factor i.e. the ratio between any two successive orders of
    magnitude and the units corresponding to each multiplication of the base
    factor.

    :param us: the unit system in use
    :return: the base factor and the list of units
    """

    units = ["K", "M", "G", "T", "P", "E", "Z", "Y"]
    if us == UnitSystem.BINARY:
        base = pow(2, 10)
        # kibi, mebi, gibi, tebi, pebi, exbi, zebi, yobi
        units = ["", *map(lambda i: f"{i}i", units)]
    else:  # us == UnitSystem.DECIMAL:
        base = pow(10, 3)
        # kilo, mega, giga, tera, peta, exa, zetta, yotta
        units = ["", *units]
    return base, units
