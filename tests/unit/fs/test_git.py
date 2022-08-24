from __future__ import annotations

from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from pls.fs.git import formatted_status, get_git_statuses


@pytest.mark.parametrize(
    "status, formatted",
    [
        ("  ", "  "),
        ("DM", "[red]D[/][yellow]M[/]"),
        (" R", "[dim]-[/][yellow]R[/]"),
        ("A ", "[green]A[/][dim]-[/]"),
        ("!!", "[dim]![/][dim]![/]"),
    ],
)
def test_formats_status(status: str, formatted: str):
    assert formatted_status(status) == formatted


@pytest.mark.parametrize(
    "entry, statuses",
    [
        ("XY file", {"file": "XY"}),
        (" Y file", {"file": " Y"}),
        ("X  file", {"file": "X "}),
        ("!! file", {"file": "!!"}),
        ("?? file", {"file": "??"}),
        ('XY "user quoted file"', {'"user quoted file"': "XY"}),
        (" R new_file\0old_file", {"new_file": " R"}),
        ("XY file with space", {"file with space": "XY"}),
        (
            " R new file with space\0old file with space",
            {"new file with space": " R"},
        ),
    ],
)
def test_statuses_handles_all_cases(entry: str, statuses: dict[Path, str]):
    entries = [entry]
    exec_git = MagicMock(return_value=MagicMock(stdout="\0".join(entries)))
    with patch("pls.fs.git.exec_git", exec_git):
        git_statuses = get_git_statuses(Path("."))
    assert git_statuses == {Path(key): value for key, value in statuses.items()}


def test_statuses_combines_two_git_cmds():
    common_entries = [
        " A not_updated",
        "MM updated_index_updated_worktree",
        "D  deleted_index",
        " R new_worktree\0renamed_worktree",
    ]
    # git status --porcelain -z --untracked-files
    u = MagicMock(
        stdout="\0".join(
            [
                *common_entries,
                "?? untracked_dir/untracked_file",
            ]
        )
    )
    # git status --porcelain -z --untracked-files=normal --ignored=matching
    u_normal_i_matching = MagicMock(
        stdout="\0".join(
            [
                *common_entries,
                "!! ignored_dir/",
                "?? untracked_dir/",
            ]
        )
    )
    exec_git = MagicMock(side_effect=[u, u_normal_i_matching])

    with patch("pls.fs.git.exec_git", exec_git):
        statuses = get_git_statuses(Path("."))

    assert len(statuses) == 7
    assert statuses == {
        Path("not_updated"): " A",
        Path("updated_index_updated_worktree"): "MM",
        Path("deleted_index"): "D ",
        Path("new_worktree"): " R",
        Path("ignored_dir/"): "!!",
        Path("untracked_dir/untracked_file"): "??",
        Path("untracked_dir/"): "??",
    }
