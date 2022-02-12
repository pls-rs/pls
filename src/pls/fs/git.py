from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Union

from pls.exceptions import ExecException


def exec_git(cmd_args: list[str], cwd: Path) -> subprocess.CompletedProcess:
    """
    Execute a ``git`` command from the given directory.

    :param cmd_args: the args to pass in the Git command
    :param cwd: the directory in which to run the command
    :return: the ``CompletedProcess`` instance
    """

    if cmd_args is None:
        cmd_args = []
    cmd = ["git", *cmd_args]
    proc = subprocess.run(
        cmd,
        check=True,
        cwd=cwd,
        capture_output=True,
        text=True,
    )
    return proc


def get_git_root(working_dir: Path) -> Union[Path, None]:
    """
    Identify the Git root for the working directory. To get the root directory,
    this uses following command::

        git rev-parse --show-toplevel

    :return: the root dir ``Path`` if Git-managed, ``None`` otherwise
    """

    try:
        proc = exec_git(
            ["rev-parse", "--show-toplevel"],
            cwd=working_dir,
        )
        root_path = Path(proc.stdout.rstrip())
        return root_path
    except subprocess.CalledProcessError:
        return None


def get_git_statuses(git_root: Path) -> dict[Path, str]:
    """
    Identify the Git statuses for all files in the working directory. To get the
    Git statues, this uses following two commands::

        git status --porcelain --untracked-files --ignored
        git status --porcelain --untracked-files=normal --ignored=matching

    Refer to the `git-status command documentation
    <https://git-scm.com/docs/git-status>`_ for more info.

    :param git_root: the root directory of the Git repository
    :return: the mapping of paths to their Git statues
    """

    status_map: dict[Path, str] = {}

    status_lines: set[str] = set()
    try:
        status = ["status", "--porcelain"]

        proc = exec_git(
            [*status, "--untracked-files", "--ignored"],
            cwd=git_root,
        )
        status_lines.update(proc.stdout.rstrip().split("\n"))

        proc = exec_git(
            [*status, "--untracked-files=normal", "--ignored=matching"],
            cwd=git_root,
        )
        status_lines.update(proc.stdout.rstrip().split("\n"))
    except subprocess.CalledProcessError:
        return status_map

    for line in status_lines:
        status = line[0:2]

        components: list[str] = line[3:].split(" ")
        if len(components) == 1:
            path = components[0]
        elif len(components) == 3:
            _, __, path = components
        else:
            raise ExecException("Could not parse Git status code")
        path = Path(path)

        status_map[path] = status

    return status_map
