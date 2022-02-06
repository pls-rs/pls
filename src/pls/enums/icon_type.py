from enum import auto

from pls.enums.base import AutoEnum


class IconType(AutoEnum):
    """
    The app supports three modes of icons. An icon can be either of these:

    - an emoji, which is supported by most modern OSes and terminals
    - a Nerd Font icon, which has more glyphs but needs separate installation
    - none, for plain output without fancy embellishments

    This enum lists these possibilities.
    """

    NERD = auto()
    EMOJI = auto()
    NONE = auto()
