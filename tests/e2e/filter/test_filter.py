from __future__ import annotations

from pathlib import Path

import pytest

from tests.e2e.utils import run_pls


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
