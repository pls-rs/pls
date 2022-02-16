import os
from typing import Literal
from unittest.mock import MagicMock, patch

import pytest

from pls.enums.unit_system import UnitSystem
from pls.fs.stats import (
    get_formatted_group,
    get_formatted_links,
    get_formatted_perms,
    get_formatted_size,
    get_formatted_time,
    get_formatted_user,
)
from tests.unit.utils import strip_formatting


@pytest.mark.parametrize(
    "nlink, mode, text",
    [
        (1, 0o40000, "1"),
        (2, 0o40000, "2"),
        (1, 0o100000, "1"),
    ],
)
def test_does_not_format_dirs_and_files_with_one_link(nlink, mode, text):
    mock_stat = MagicMock(st_nlink=nlink, st_mode=mode)
    assert get_formatted_links(mock_stat) == text


def test_formats_files_with_more_than_one_link():
    mock_stat = MagicMock(st_nlink=2, st_mode=0o100000)
    assert get_formatted_links(mock_stat) == "[yellow]2[/]"


@pytest.mark.parametrize(
    "mode, text",
    [
        (0o777, "rwx rwx rwx"),
        (0o654, "rw- r-x r--"),
        (0o321, "-wx -w- --x"),
        (0o000, "--- --- ---"),
    ],
)
def test_gets_correct_permission_text_for_rwx(mode: int, text: str):
    mock_stat = MagicMock(st_mode=mode)
    assert strip_formatting(get_formatted_perms(mock_stat)) == text


@pytest.mark.parametrize(
    "special_bit, text",
    [
        (0o0, "rwx rwx rwx"),
        (0o1, "rwx rwx rwt"),
        (0o2, "rwx rws rwx"),
        (0o4, "rws rwx rwx"),
        (0o7, "rws rws rwt"),
    ],
)
def test_gets_correct_permission_text_for_rwx_with_special_bit(
    special_bit: int, text: str
):
    mode = special_bit * 0o1000 + 0o777
    mock_stat = MagicMock(st_mode=mode)
    assert strip_formatting(get_formatted_perms(mock_stat)) == text


@pytest.mark.parametrize(
    "special_bit, text",
    [
        (0o0, "rw- rw- rw-"),
        (0o1, "rw- rw- rwT"),
        (0o2, "rw- rwS rw-"),
        (0o4, "rwS rw- rw-"),
        (0o7, "rwS rwS rwT"),
    ],
)
def test_gets_correct_permission_text_for_rw_with_special_bit(
    special_bit: int, text: str
):
    mode = special_bit * 0o1000 + 0o666
    mock_stat = MagicMock(st_mode=mode)
    assert strip_formatting(get_formatted_perms(mock_stat)) == text


def test_user_is_dimmed_if_not_current():
    mock_getpwuid = MagicMock(return_value=MagicMock(pw_name="x"))
    mock_state = MagicMock(username="y")
    mock_stat = MagicMock(st_uid=None)
    with patch("pwd.getpwuid", mock_getpwuid), patch("pls.fs.stats.state", mock_state):
        assert get_formatted_user(mock_stat) == "[dim]x[/]"


def test_group_is_dimmed_if_not_current():
    mock_getgrgid = MagicMock(return_value=MagicMock(gr_name="x"))
    mock_state = MagicMock(groups={"y", "z"})
    mock_stat = MagicMock(st_gid=None)
    with patch("grp.getgrgid", mock_getgrgid), patch("pls.fs.stats.state", mock_state):
        assert get_formatted_group(mock_stat) == "[dim]x[/]"


@pytest.mark.parametrize(
    "size, text",
    [
        (pow(2, 00), "1  B"),
        (pow(2, 10), "1KiB"),
        (pow(2, 20), "1MiB"),
        (pow(2, 30), "1GiB"),
    ],
)
def test_gets_correct_binary_size(size: int, text: str):
    mock_stat = MagicMock(st_size=size, st_mode=0o100000)
    assert strip_formatting(get_formatted_size(mock_stat)) == text


@pytest.mark.parametrize(
    "size, text",
    [
        (pow(10, 0), "1 B"),
        (pow(10, 3), "1KB"),
        (pow(10, 6), "1MB"),
        (pow(10, 9), "1GB"),
    ],
)
def test_gets_correct_decimal_size(size: int, text: str):
    mock_stat = MagicMock(st_size=size, st_mode=0o100000)
    mock_args = MagicMock(units=UnitSystem.DECIMAL)
    with patch("pls.fs.stats.args", mock_args):
        assert strip_formatting(get_formatted_size(mock_stat)) == text


@pytest.mark.parametrize(
    "size, text",
    [
        (pow(10, 0), "1B"),
        (pow(10, 3), "1000B"),
        (pow(10, 6), "1000000B"),
        (pow(10, 9), "1000000000B"),
    ],
)
def test_gets_correct_raw_size(size: int, text: str):
    mock_stat = MagicMock(st_size=size, st_mode=0o100000)
    mock_args = MagicMock(units=UnitSystem.NONE)
    with patch("pls.fs.stats.args", mock_args):
        assert strip_formatting(get_formatted_size(mock_stat)) == text


def test_shows_blank_size_for_directory():
    mock_stat = MagicMock(st_size=1, st_mode=0o040000)
    assert strip_formatting(get_formatted_size(mock_stat)) == "-"


@pytest.mark.parametrize(
    "attr_name",
    ["st_ctime", "st_mtime", "st_atime"],
)
def test_formats_timestamp_in_local_tz(
    attr_name: Literal["st_ctime", "st_mtime", "st_atime"]
):
    mock_stat = MagicMock(st_ctime=0, st_mtime=0, st_atime=0)
    exp = "1970-01-01 04:00:00 "
    with patch.dict(os.environ, {"TZ": "Asia/Dubai"}, clear=True):
        assert strip_formatting(get_formatted_time(mock_stat, attr_name)) == exp


@pytest.mark.parametrize(
    "time_fmt, formatted_time",
    [
        (
            "[red]%Y[/]-[green]%m[/]-[blue]%d[/] %H:%M[dim]:%S[/] %p",
            "[red]2069[/]-[green]04[/]-[blue]20[/] 04:20[dim]:00[/] AM",
        ),
    ],
)
def test_formats_timestamp_as_asked(time_fmt, formatted_time):
    mock_args = MagicMock(time_fmt=time_fmt)
    mock_stat = MagicMock(st_ctime=3_133_637_400)
    with patch("pls.fs.stats.args", mock_args), patch.dict(
        os.environ, {"TZ": "Asia/Kolkata"}, clear=True
    ):
        assert get_formatted_time(mock_stat, "st_ctime") == formatted_time
