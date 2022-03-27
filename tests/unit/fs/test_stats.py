from sys import platform
from typing import Literal
from unittest.mock import MagicMock, patch

import pytest
from freezegun import freeze_time

from pls.enums.unit_system import UnitSystem
from pls.fs.stats import (
    get_formatted_group,
    get_formatted_links,
    get_formatted_perms,
    get_formatted_size,
    get_formatted_time,
    get_formatted_user,
)
from pls.globals import args, state
from pls.utils.strip_fmt import strip_formatting


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


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_user_is_dimmed_if_not_current():
    mock_getpwuid = MagicMock(return_value=MagicMock(pw_name="x"))
    mock_stat = MagicMock(st_uid=219)
    with patch("pwd.getpwuid", mock_getpwuid), patch.multiple(state.state, uid=49):
        assert get_formatted_user(mock_stat) == "[dim]x[/]"


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_gone_user_is_shown_as_int():
    mock_getpwuid = MagicMock(side_effect=KeyError("uid not found"))
    mock_stat = MagicMock(st_uid=123)
    with patch("pwd.getpwuid", mock_getpwuid):
        assert get_formatted_user(mock_stat) == "[red dim]123[/]"


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_group_is_dimmed_if_not_current():
    mock_getgrgid = MagicMock(return_value=MagicMock(gr_name="x"))
    mock_stat = MagicMock(st_gid=0)
    with patch("grp.getgrgid", mock_getgrgid), patch.multiple(state.state, gids={1, 2}):
        assert get_formatted_group(mock_stat) == "[dim]x[/]"


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_gone_group_is_shown_as_int():
    mock_getgrgid = MagicMock(side_effect=KeyError("gid not found"))
    mock_stat = MagicMock(st_gid=123)
    with patch("grp.getgrgid", mock_getgrgid):
        assert get_formatted_group(mock_stat) == "[red dim]123[/]"


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
    with patch.multiple(args.args, units=UnitSystem.DECIMAL):
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
    with patch.multiple(args.args, units=UnitSystem.NONE):
        assert strip_formatting(get_formatted_size(mock_stat)) == text


def test_shows_blank_size_for_directory():
    mock_stat = MagicMock(st_size=1, st_mode=0o040000)
    assert strip_formatting(get_formatted_size(mock_stat)) == "-"


@pytest.mark.parametrize(
    "attr_name",
    ["st_ctime", "st_mtime", "st_atime"],
)
@freeze_time(tz_offset=+4.0)
def test_formats_timestamp_in_local_tz(
    attr_name: Literal["st_ctime", "st_mtime", "st_atime"]
):
    mock_stat = MagicMock(st_ctime=0, st_mtime=0, st_atime=0)
    exp = "1970-01-01 04:00:00 "
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
@freeze_time(tz_offset=+5.5)
def test_formats_timestamp_as_asked(time_fmt, formatted_time):
    mock_stat = MagicMock(st_ctime=3_133_637_400)
    with patch.multiple(args.args, time_fmt=time_fmt):
        assert get_formatted_time(mock_stat, "st_ctime") == formatted_time
