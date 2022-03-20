from sys import platform
from unittest.mock import MagicMock

import pytest

from pls.globals import state


pytestmark = pytest.mark.skipif(
    platform == "win32", reason="Feature unsupported on Windows"
)


def test_uid(pwd_grp):
    state.state.setup_user_groups()
    assert state.state.uid == 219


def test_gids(pwd_grp):
    state.state.setup_user_groups()
    assert state.state.gids == {0, 1, 2}


def test_when_user_does_not_exist(pwd_grp):
    pwd, grp = pwd_grp
    pwd.getpwuid = MagicMock(side_effect=KeyError)
    state.state.setup_user_groups()
    assert state.state.uid == 219
    assert state.state.gids == set()
