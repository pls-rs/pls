import logging
import os
from typing import Union

from rich.logging import RichHandler

rich_handler = RichHandler(
    level=logging.WARNING,  # By default, only warnings and higher levels are handled
    show_time=False,
    show_path=False,
    markup=True,
)

logging.basicConfig(
    level=logging.DEBUG,
    format="[bold]%(name)s[/] - %(message)s",
    handlers=[rich_handler],
)

logger = logging.getLogger(__name__)


def configure_log_level(level: Union[str, int, None] = None):
    """
    Configures the global log handler so that all loggers obtained using ``getLogger``
    are automatically being properly handled.

    :param level: the level of the global handler below which logs are not shown
    """

    if level is None:
        if level := os.getenv("PLS_LOG_LEVEL"):
            try:
                level = int(level)
            except ValueError:  # not a number
                pass
        else:
            level = "WARNING"

    rich_handler.setLevel(level)
    logger.info(f"Logging configured with level [italic]{level}[/]")
