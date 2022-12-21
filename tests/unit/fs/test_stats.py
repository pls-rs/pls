from sys import platform
from unittest.mock import MagicMock, patch

import pytest
from freezegun import freeze_time

from pls.enums.node_type import NodeType
from pls.enums.unit_system import UnitSystem
from pls.fs.stats import (get_formatted_group, get_formatted_links,
                          get_formatted_perms, get_formatted_size,
                          get_formatted_time, get_formatted_user)
from pls.globals import args, state
from pls.utils.strip_fmt import strip_formatting


@pytest.mark.parametrize(
    "node_type, st_nlink, text",
    [
        (NodeType.FILE, 1, "1"),
        (NodeType.DIR, 1, "1"),
        (NodeType.DIR, 2, "2"),
    ],
)
def test_does_not_format_dirs_and_files_with_one_link(
    node_type: NodeType, st_nlink: int, text: str
):
    assert get_formatted_links(node_type, st_nlink) == text


@pytest.mark.parametrize(
    "node_type",
    [
        NodeType.FIFO,
        NodeType.SOCKET,
        NodeType.CHAR_DEVICE,
        NodeType.BLOCK_DEVICE,
        NodeType.FILE,
    ],
)
def test_formats_files_with_more_than_one_link(node_type):
    assert get_formatted_links(node_type, 2) == "[yellow]2[/]"


def test_formatted_links_are_cached():
    get_formatted_links.cache_clear()

    get_formatted_links(NodeType.FILE, 2)
    get_formatted_links(NodeType.FILE, 2)
    get_formatted_links(NodeType.FILE, 2)

    assert get_formatted_links.cache_info().misses == 1
    assert get_formatted_links.cache_info().hits == 2


@pytest.mark.parametrize(
    "st_mode, text",
    [
        (0o777, "rwx rwx rwx"),
        (0o654, "rw- r-x r--"),
        (0o321, "-wx -w- --x"),
        (0o000, "--- --- ---"),
    ],
)
def test_gets_correct_permission_text_for_rwx(st_mode: int, text: str):
    assert strip_formatting(get_formatted_perms(st_mode)) == text


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
    assert strip_formatting(get_formatted_perms(mode)) == text


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
    assert strip_formatting(get_formatted_perms(mode)) == text


def test_formatted_perms_are_cached():
    get_formatted_perms.cache_clear()

    get_formatted_perms(0o777)
    get_formatted_perms(0o777)
    get_formatted_perms(0o777)

    assert get_formatted_perms.cache_info().misses == 1
    assert get_formatted_perms.cache_info().hits == 2


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_user_is_dimmed_if_not_current():
    get_formatted_user.cache_clear()
    mock_getpwuid = MagicMock(
        side_effect=lambda uid: MagicMock(pw_name="x" if uid == 219 else "y")
    )
    with patch("pwd.getpwuid", mock_getpwuid), patch.multiple(state.state, uid=49):
        assert get_formatted_user(219) == "[dim]x[/]"


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_gone_user_is_shown_as_int():
    get_formatted_user.cache_clear()
    mock_getpwuid = MagicMock(side_effect=KeyError("uid not found"))
    with patch("pwd.getpwuid", mock_getpwuid):
        assert get_formatted_user(219) == "[red dim]219[/]"


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_formatted_user_is_cached():
    get_formatted_user.cache_clear()

    get_formatted_user(219)
    get_formatted_user(219)
    get_formatted_user(219)

    assert get_formatted_user.cache_info().misses == 1
    assert get_formatted_user.cache_info().hits == 2


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_group_is_dimmed_if_not_current():
    get_formatted_group.cache_clear()
    mock_getgrgid = MagicMock(
        side_effect=lambda gid: MagicMock(gr_name="x" if gid == 49 else "y")
    )
    with patch("grp.getgrgid", mock_getgrgid), patch.multiple(state.state, gids={1, 2}):
        assert get_formatted_group(49) == "[dim]x[/]"


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_gone_group_is_shown_as_int():
    get_formatted_group.cache_clear()
    mock_getgrgid = MagicMock(side_effect=KeyError("gid not found"))
    with patch("grp.getgrgid", mock_getgrgid):
        assert get_formatted_group(3) == "[red dim]3[/]"


@pytest.mark.skipif(platform == "win32", reason="Feature unsupported on Windows")
def test_formatted_group_is_cached():
    get_formatted_group.cache_clear()

    get_formatted_group(219)
    get_formatted_group(219)
    get_formatted_group(219)

    assert get_formatted_group.cache_info().misses == 1
    assert get_formatted_group.cache_info().hits == 2


@pytest.mark.parametrize(
    "st_size, text",
    [
        (pow(2, 00), "1  B"),
        (pow(2, 10), "1KiB"),
        (pow(2, 20), "1MiB"),
        (pow(2, 30), "1GiB"),
    ],
)
def test_gets_correct_binary_size(st_size: int, text: str):
    with patch.multiple(args.args, units=UnitSystem.BINARY):
        assert strip_formatting(get_formatted_size(NodeType.FILE, st_size)) == text
    # Since ``UnitSystem.BINARY`` is also the default, test without ``patch``.
    assert strip_formatting(get_formatted_size(NodeType.FILE, st_size)) == text


@pytest.mark.parametrize(
    "st_size, text",
    [
        (pow(10, 0), "1 B"),
        (pow(10, 3), "1KB"),
        (pow(10, 6), "1MB"),
        (pow(10, 9), "1GB"),
    ],
)
def test_gets_correct_decimal_size(st_size: int, text: str):
    with patch.multiple(args.args, units=UnitSystem.DECIMAL):
        assert strip_formatting(get_formatted_size(NodeType.FILE, st_size)) == text


@pytest.mark.parametrize(
    "st_size, text",
    [
        (pow(10, 0), "1B"),
        (pow(10, 3), "1000B"),
        (pow(10, 6), "1000000B"),
        (pow(10, 9), "1000000000B"),
    ],
)
def test_gets_correct_raw_size(st_size: int, text: str):
    with patch.multiple(args.args, units=UnitSystem.NONE):
        assert strip_formatting(get_formatted_size(NodeType.FILE, st_size)) == text


def test_shows_blank_size_for_directory():
    assert strip_formatting(get_formatted_size(NodeType.DIR, 1)) == "-"


@freeze_time(tz_offset=+4.0)
def test_formats_timestamp_in_local_tz():
    exp = "1970-01-01 04:00:00 "
    assert strip_formatting(get_formatted_time(0)) == exp


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
    with patch.multiple(args.args, time_fmt=time_fmt):
        assert get_formatted_time(3_133_637_400) == formatted_time


def test_shows_blank_time_if_timestamp_is_none():
    assert get_formatted_time(None) == ""
