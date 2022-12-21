import argparse
import logging
import os

from pls.globals import console
from pls.output.update import check_update, print_version

logger = logging.getLogger(__name__)


class BooleanOptionalAction(argparse.Action):
    """
    Allows setting the boolean toggles using two option flags `--x` and `--no-x`. This
    action overrides ``nargs`` set on the argument.

    This exists in Python 3.9. TODO: Remove after the upgrade.
    """

    def __init__(self, option_strings, *args, **kwargs):
        kwargs.pop("nargs", None)

        _option_strings = []
        for option_string in option_strings:
            _option_strings.append(option_string)

            if option_string.startswith("--"):
                _option_strings.append(f"--no-{option_string[2:]}")

        super().__init__(*args, **kwargs, nargs=0, option_strings=_option_strings)

    def __call__(self, parser, namespace, values, option_string=None):
        if option_string in self.option_strings:
            setattr(namespace, self.dest, not option_string.startswith("--no-"))


class CollectOrClearAction(argparse.Action):
    """
    Extends the behaviour of the built-in append action to provide a value ``"none"``
    that can clear the array and reset it.

    The order of arguments is important.

    - ``-x none -x a -x c`` gives ``x`` = ["a", "c"];
      ``x`` is reset, then appended "a" and "c".
    - ``-x a -x none -x c`` gives ``x`` = ["c"];
      ``x`` is appended "a", then reset, then appended "c".
    """

    def __call__(self, parser, namespace, values, option_string=None):
        items = getattr(namespace, self.dest, None)
        if values == "none":
            items = []
        else:
            items = items[:] if isinstance(items, list) else []
            items.append(values)
        setattr(namespace, self.dest, items)


class StoreOrCountAction(argparse.Action):
    """
    Combines the behavior of the built-in store and count actions to provide the option
    to set the count directly via an argument instead of repeating the flag. This action
    overrides ``type`` and ``nargs`` set on the argument.

    The order of arguments is important.

    - ``-x 1 -x -x`` gives ``x`` = 3;
      ``x`` is set to 1, then incremented twice.
    - ``-x -x -x 1`` gives ``x`` = 1;
      ``x`` is incremented twice, then set to 1 (which voids the increments).
    """

    def __init__(self, *args, **kwargs):
        kwargs.pop("type", None)
        kwargs.pop("nargs", None)

        super().__init__(*args, **kwargs, type=int, nargs=argparse.OPTIONAL)

    def __call__(self, parser, namespace, values, option_string=None):
        if values is not None:  # ``values`` can be 0
            setattr(namespace, self.dest, values)
        else:
            count = getattr(namespace, self.dest, None)
            if count is None:
                count = 0
            setattr(namespace, self.dest, count + 1)


class VersionUpdateAction(argparse.Action):
    """
    Combines the behaviour of the built inCheck if there are any new versions of
    ``pls`` available on PyPI.
    """

    def __init__(self, *args, **kwargs):
        kwargs.pop("nargs", None)

        super().__init__(*args, **kwargs, nargs=0)

    def __call__(self, parser, *args, **kwargs):
        console.console = console.get_console()

        print_version()
        if not (os.getenv("PLS_NO_UPDATE_CHECK") or os.getenv("CI")):
            check_update()

        parser.exit()


def register_actions(parser: argparse.ArgumentParser):
    parser.register("action", "boolean_optional", BooleanOptionalAction)
    parser.register("action", "collect_or_clear", CollectOrClearAction)
    parser.register("action", "store_or_count", StoreOrCountAction)
    parser.register("action", "version_update", VersionUpdateAction)
