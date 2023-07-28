import os
import subprocess
from pathlib import Path

PROJECT_ROOT = project_root = Path(__file__).parents[4]


def run_cmd(args: list[str], **kwargs) -> subprocess.CompletedProcess:
    """
    Run the given command and return the completed process.

    All commands are executed from the working directory of the project root.

    All keyword arguments are forwarded to ``subprocess.run``. If the ``env``
    keyword argument is provided, it will be merged into the system environment
    variables before forwarding.

    :param args: the argument vector to execute
    :param kwargs: keyword arguments to forward to ``subprocess.run``
    :return: the completed process, irrespective of success or failure
    """

    proc = subprocess.run(
        args,
        cwd=PROJECT_ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        env=os.environ.copy() | kwargs.pop("env", {}),
        **kwargs,
    )
    if proc.returncode != 0:
        print(proc.stdout)
        print(proc.stderr)
    return proc
