from __future__ import annotations

import argparse
import re
from enum import Enum
from sys import platform, stderr
from typing import IO, Optional

from rich import print as rich_print
from rich.markup import escape

from pls.utils.strip_fmt import strip_formatting


class PlsFormatter(argparse.HelpFormatter):
    """
    This formatter extends ``argparse.HelpFormatter`` to use Rich markup in the output.
    """

    def __init__(self, *args, **kwargs):
        # Update ``max_help_position`` as needed to fit the arguments in one line.
        super().__init__(*args, **kwargs, max_help_position=28)

    class _Section(argparse.HelpFormatter._Section):  # type: ignore
        def __init__(self, formatter, parent, heading=None):
            super().__init__(formatter, parent, heading)

            if isinstance((heading := getattr(self, "heading", None)), str):
                self.heading = f"[bold]{heading.upper()}[/]"

    def _format_usage(self, *args, **kwargs) -> str:
        """
        Use Rich's ``escape`` function to ensure that all the square brackets in the
        usage text are printed to the screen.
        """

        usage = super()._format_usage(*args, **kwargs)

        # Handle rested brackets
        usage = re.sub(r"\[(?P<out>.*)\[(?P<in>.*)]]", r"[\g<out>\\[\g<in>]]", usage)

        return escape(usage)

    def _format_action_invocation(self, action: argparse.Action) -> str:
        if not action.option_strings:
            default = self._get_default_metavar_for_positional(action)
            (metavar,) = self._metavar_formatter(action, default)(1)
            return f"[blue]{metavar}[/]"

        parts = []

        opts = sorted(action.option_strings, key=lambda item: len(item), reverse=True)
        if len(opts) == 2 and opts[0].startswith("--no"):
            complete_arg_name = f"[cyan]--[magenta italic](no-)[/]{opts[1][2:]}[/]"
        else:
            complete_arg_name = "/".join(f"[cyan]{opt}[/]" for opt in opts)
        parts.append(complete_arg_name)

        if action.nargs != 0:
            # Option takes a value; add that to the output.
            default = self._get_default_metavar_for_optional(action)
            args_string = self._format_args(action, default)
            parts.append(args_string)

        return " ".join(parts)

    def _action_choices(self, action):
        """
        Present the choices of an action as a string.

        :param action: the action for which to present the string of choices
        :return: the string representation of an action's choices
        """

        choices = action.choices
        if choices is None:
            return None
        choice_str = [
            choice.value if isinstance(choice, Enum) else choice for choice in choices
        ]
        return f"[bold]Choices[/]: {choice_str}"

    def _format_action(self, action):
        # Width and spacing measurements
        help_position = min(self._action_max_length + 2, self._max_help_position)
        help_width = max(self._width - help_position, 11)
        action_width = help_position - self._current_indent - 2

        action_header = self._format_action_invocation(action)
        action_header_no_fmt = strip_formatting(action_header)
        extra_chars = len(action_header) - len(action_header_no_fmt)

        indent = " " * self._current_indent

        if not action.help:
            # No help; start on same line and add a final newline.
            action_header = f"{indent}{action_header}\n"
            indent_first = 0  # unused
        elif len(action_header_no_fmt) <= action_width:
            # Short action name; start on the same line and pad two spaces.
            action_header = f"{indent}{action_header:{action_width+extra_chars}}  "
            indent_first = 0
        else:
            # Long action name; start on the next line.
            action_header = f"{indent}{action_header}\n"
            indent_first = help_position

        # Collect the pieces of the action help
        parts = [action_header]

        if action.help:
            bits = [self._expand_help(action), self._action_choices(action)]
            for index, bit in enumerate(bits):
                if bit is None:
                    continue

                # Add lines of help text.
                help_lines = self._split_lines(bit, help_width)

                first_indent = indent_first if index == 0 else help_position
                sep = "[dim]│[/] " if platform != "win32" else ""

                parts.append(f"{' '*first_indent}{sep}{help_lines[0]}\n")
                parts.extend(
                    f"{' '*help_position}{sep}{line}\n" for line in help_lines[1:]
                )
        elif not action_header.endswith("\n"):
            # Add a newline if the description doesn't end with one.
            parts.append("\n")

        # If there are any sub-actions, add their help as well.
        for sub_action in self._iter_indented_subactions(action):
            parts.append(self._format_action(sub_action))

        # Combine help parts.
        return self._join_parts(parts)


class PlsParser(argparse.ArgumentParser):
    """
    This parser extends ``argparse.ArgumentParser`` to use Rich for printing the output.
    This allows help text and descriptions to use formatting and colors for better UX.
    """

    def _print_message(self, message: str, file: Optional[IO[str]] = None):
        if message:
            if file is None:
                file = stderr
            rich_print(message, file=file)
