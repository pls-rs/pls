from pathlib import Path
from sys import platform
from unittest.mock import MagicMock

import pytest

from pls.globals import state


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_uid(pwd_grp):
    state.state.setup_user_groups()
    assert state.state.uid == 219


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_gids(pwd_grp):
    state.state.setup_user_groups()
    assert state.state.gids == {0, 1, 2}


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_when_user_does_not_exist(pwd_grp):
    pwd, grp = pwd_grp
    pwd.getpwuid = MagicMock(side_effect=KeyError)
    state.state.setup_user_groups()
    assert state.state.uid == 219
    assert state.state.gids == set()


@pytest.mark.parametrize(
    "name, status",
    [
        ("file_01", "??"),  # untracked
        ("file_02", "!!"),  # ignored
        ("file_04", " M"),  # committed, changed
        ("file_05", "MM"),  # committed, changed, added, changed
        ("file_76", "R "),  # committed, git moved
        ("file_07", " D"),  # committed, deleted
        ("file_08", "A "),  # added
        ("file_09", "AM"),  # added, changed
        ("file_10", "AD"),  # added, deleted
    ],
)
def test_unclean_files_have_correct_git_status(name: str, status: str, git_area: Path):
    node = Path(name)
    state.state.setup_git(git_area)
    assert state.state.git_status_map[node] == status


def test_clean_files_have_no_git_status(git_area: Path):
    node = Path("file_03")  # committed
    state.state.setup_git(git_area)
    assert node not in state.state.git_status_map
