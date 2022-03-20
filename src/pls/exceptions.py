import sys
from typing import Optional

from rich.console import Console
from rich.pretty import Pretty


console = Console()  # This module does not use ``pls.globals.console.console``.


class PlsException(Exception):
    """
    This is the base class for all exceptions raised by `pls`. This implements many
    useful utilities for printing error messages.
    """

    def print(self):
        """
        Pretty-print the exception class and message to the console.
        """

        console.print(f"[red bold]{type(self).__name__}[/]: {self}")


class ConfigException(PlsException):
    """
    These exceptions occur as a result of invalid configuration and are largely
    internal. The user should not experience these exceptions unless they are
    modifying the configs.

    The output of these exceptions must be detailed and developer-oriented.
    """

    def __init__(self, *args, fail_spec: Optional[dict] = None):
        super().__init__(*args)
        self.fail_obj = fail_spec

    def print(self):
        super().print()

        if self.fail_obj:
            console.print("Errant node spec:")
            console.print(
                Pretty(
                    self.fail_obj, indent_guides=False, expand_all=True, indent_size=2
                )
            )


class ExecException(PlsException):
    """
    These exceptions occur as a result of bad input and cause the application to
    terminate. These exceptions will be seen by the user.

    The output of these exceptions must be succinct and helpful.
    """

    pass


class ArgException(ExecException):
    """
    These exceptions are a result of invalid arguments.
    """

    def __init__(self, *args, arg_name: str):
        super().__init__(*args)
        self.arg_name = arg_name

    def print(self):
        console.print(
            f"[red bold]{type(self).__name__}[/] in [bold]{self.arg_name}[/]: {self}"
        )


def hook(exc_type, exc, traceback):
    """
    This function intercepts arguments before they reach the system handler, thus
    handling `PlsExceptions`` by showing a clear and succinct error message.

    :param exc_type: the type of the exception raised
    :param exc: the exception instance
    :param traceback: the traceback of the exception
    """

    if isinstance(exc, PlsException):
        exc.print()
    else:
        sys.__excepthook__(exc_type, exc, traceback)


sys.excepthook = hook
