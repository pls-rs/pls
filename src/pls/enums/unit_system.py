from __future__ import annotations

import logging
from enum import auto
from functools import cached_property

from pls.enums.base import AutoEnum


logger = logging.getLogger(__name__)


class UnitSystem(AutoEnum):
    """
    This enum lists the different unit systems in which measurements can be represented,
    also including an option to not use any unit system at all.
    """

    BINARY = auto()  # powers of 2 (steps of 2^10) are used
    DECIMAL = auto()  # powers of 10 (steps of 10^3) are used
    NONE = auto()  # no higher units

    @cached_property
    def base_pad_units(self) -> tuple[int, int, list[str]]:
        """
        Get the base factor i.e. the ratio between any two successive orders of
        magnitude and the units corresponding to each multiplication of the base
        factor.

        :return: the base factor, length of each unit string and the list of units
        """

        if self == UnitSystem.NONE:
            return 1, 0, [""]

        logger.debug(f"Unit system: {self}")
        units = [
            "K",  # kibi | kilo
            "M",  # mebi | mega
            "G",  # gibi | giga
            "T",  # tebi | tera
            "P",  # pebi | peta
            "E",  # exbi | exa
            "Z",  # zebi | zetta
            "Y",  # yobi | yotta
        ]

        if self == UnitSystem.DECIMAL:
            base = pow(10, 3)
            pad = 2  # units will be 2 chars e.g. KB, GB
            units = ["", *units]
        else:  # self == UnitSystem.BINARY (default)
            base = pow(2, 10)
            pad = 3  # units will be 3 chars e.g. KiB, GiB
            units = ["", *map(lambda i: f"{i}i", units)]
        return base, pad, units
