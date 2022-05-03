from __future__ import annotations

from pathlib import Path

import pytest

from tests.e2e.utils import run_pls


@pytest.mark.parametrize(
    "args, visible, invisible",
    [
        ([], (-1, 1), (-3, -2)),
        (["-a"], (-2, 1), (-3, -3)),
        (["--all"], (-2, 1), (-3, -3)),
        (["-a", "-a"], (-3, 1), ()),
        (["--all", "--all"], (-3, 1), ()),
        (["-a", "-1"], (0, 1), (-3, -1)),
        (["--all", "-1"], (0, 1), (-3, -1)),
    ],
)
def test_all(
    args: list[str],
    visible: tuple[int, int],
    invisible: tuple[int, int],
    imp_workbench: Path,
):
    def file_name(idx: int) -> str:
        return str(idx) if idx >= 0 else f"_{abs(idx)}"

    args.insert(0, str(imp_workbench))
    proc = run_pls(args)
    for (node_set, presence_cond) in [(visible, True), (invisible, False)]:
        if not node_set:
            continue
        ll, ul = node_set
        for index in range(ll, ul + 1):
            assert (file_name(index) in proc.stdout) == presence_cond


@pytest.mark.parametrize(
    "args",
    [
        [],
        ["--dirs"],
    ],
)
def test_dirs(args: list[str], type_workbench: Path):
    args.append(str(type_workbench))
    proc = run_pls(args)
    for name in ["dir_a", "dir_b"]:
        assert name in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["--no-dirs"],
    ],
)
def test_no_dirs(args: list[str], type_workbench: Path):
    args.append(str(type_workbench))
    proc = run_pls(args)
    for name in ["dir_a", "dir_b"]:
        assert name not in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        [],
        ["--files"],
    ],
)
def test_files(args: list[str], type_workbench: Path):
    args.append(str(type_workbench))
    proc = run_pls(args)
    for name in ["file_a", "file_b"]:
        assert name in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["--no-files"],
    ],
)
def test_no_files(args: list[str], type_workbench: Path):
    args.append(str(type_workbench))
    proc = run_pls(args)
    for name in ["file_a", "file_b"]:
        assert name not in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["--exclude", r".*a"],
        ["-e", r".*a"],
    ],
)
def test_exclude(args: list[str], pattern_workbench: Path):
    args.append(str(pattern_workbench))
    proc = run_pls(args)
    assert "bc" in proc.stdout
    for name in ["ab", "ca"]:
        assert name not in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["--only", r".*a"],
        ["-o", r".*a"],
    ],
)
def test_only(args: list[str], pattern_workbench: Path):
    args.append(str(pattern_workbench))
    proc = run_pls(args)
    assert "bc" not in proc.stdout
    for name in ["ab", "ca"]:
        assert name in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["--exclude", r".*b", "--only", r".*c"],
        ["-e", r".*b", "-o", r".*c"],
    ],
)
def test_exclude_only(args: list[str], pattern_workbench: Path):
    args.append(str(pattern_workbench))
    proc = run_pls(args)
    assert "ca" in proc.stdout
    for name in ["ab", "bc"]:
        assert name not in proc.stdout
