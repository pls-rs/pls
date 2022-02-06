from enum import auto

from pls.enums.base import AutoEnum


class SortOrder(AutoEnum):
    """
    Nodes can be sorted in two orders:

    - ascending
    - descending

    This enum lists these possibilities.
    """

    ASC = auto()
    DESC = auto()
