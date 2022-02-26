from __future__ import annotations

from typing import Tuple, cast

from rich.terminal_theme import TerminalTheme

from pls.data.utils import internal_yml_path, load_yaml_file


def get_color(name) -> tuple[int, int, int]:
    """
    Get the RBG tuple for the color with the given name.

    :param name: the name of the color for which to get the RGB values
    :return: a tuple with the red, green and blue channel values
    """

    solarized = load_yaml_file(internal_yml_path("../data/solarized.yml"))
    color = solarized[name]
    return cast(Tuple[int, int, int], tuple(color))


def get_terminal_theme():
    """
    Get the ``TerminalTheme`` instance to use with Rich. This is useful when
    writing the console output to a file where Rich cannot infer the active
    theme variables.

    :return: a populated ``TerminalTheme`` instance
    """

    colors = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
    ]
    return TerminalTheme(
        background=get_color("brblack"),  # base 03
        foreground=get_color("brblue"),  # base 0
        normal=[get_color(color) for color in colors],
        bright=[get_color(f"br{color}") for color in colors],
    )


solarized_theme = get_terminal_theme()
"""the terminal theme used by the console during exporting"""
