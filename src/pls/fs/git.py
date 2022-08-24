from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Optional


def formatted_status(status: str) -> str:
    """
    Get the given Git status formatted using Rich console markup. Expects the
    two-letter Git status as returned by git-status with the ``--porcelain``
    flag.

    :param status: the status to format
    :return: the formatted Git status
    """

    if status == "  ":
        return status

    format_map: dict[str, str] = {
        "D": "red",  # deleted
        "M": "yellow",  # modified
        "R": "yellow",  # renamed
        "A": "green",  # added
        "!": "dim",  # ignored
        "-": "dim",  # padding
    }
    fmt_status = ""
    for letter in status:
        if letter == " ":
            letter = "-"
        if letter in format_map:
            fmt_status = f"{fmt_status}[{format_map[letter]}]{letter}[/]"
        else:
            fmt_status = f"{fmt_status}{letter}"
    return fmt_status


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


def get_git_root(working_dir: Path) -> Optional[Path]:
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
    except (subprocess.CalledProcessError, FileNotFoundError):
        return None


def _split_git_output(stdout: str) -> list[str]:
    """
    Split the output of Git status with the ``--porcelain`` and ``-z`` flags. The
    NUL character ``\0`` is used as the separator. Renamed files need to be handled
    differently due to the presence of two file names in one line.

    :param stdout: the output of the Git subprocess, separated with NUL characters
    :return: the status list
    """

    skip: bool = False
    lines: list[str] = []
    for status in stdout.rstrip().split("\0"):
        if skip:
            skip = False
            continue
        if "R" in status:
            skip = True
        if status:
            lines.append(status)
    return lines


def get_git_statuses(git_root: Path) -> dict[Path, str]:
    """
    Identify the Git statuses for all files in the working directory. To get the
    Git statues, this uses following two commands::

        git status --porcelain -z --untracked-files
        git status --porcelain -z --untracked-files=normal --ignored=matching

    Refer to the `git-status command documentation
    <https://git-scm.com/docs/git-status>`_ for more info.

    :param git_root: the root directory of the Git repository
    :return: the mapping of paths to their Git statues
    """

    status_map: dict[Path, str] = {}

    status_lines: set[str] = set()
    try:
        status_args = ["status", "--porcelain", "-z"]

        proc = exec_git(
            [*status_args, "--untracked-files"],
            cwd=git_root,
        )
        if proc.stdout:
            status_lines.update(_split_git_output(proc.stdout.rstrip()))

        proc = exec_git(
            [*status_args, "--untracked-files=normal", "--ignored=matching"],
            cwd=git_root,
        )
        if proc.stdout:
            status_lines.update(_split_git_output(proc.stdout.rstrip()))
    except (subprocess.CalledProcessError, FileNotFoundError):
        return status_map

    for line in status_lines:
        status = line[0:2]
        path = Path(line[3:].strip())

        status_map[path] = status

    return status_map
