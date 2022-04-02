import argparse
import logging


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


class StoreOrCountAction(argparse.Action):
    """
    Combines the behavior of the built-in store and count actions to provide the option
    to set the count directly via an argument instead of repeating the flag. This action
    overrides ``type`` and ``nargs`` set on the argument.

    | The order of arguments is important.
    | ``-a0 -a -a`` gives ``a`` = 2; ``a`` is set to 0, then incremented by 2.
    | ``-a -a -a1`` gives ``a`` = 1; ``a`` is incremented by 2, then set to 1.
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
