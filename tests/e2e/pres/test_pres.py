from __future__ import annotations

import re
from pathlib import Path

import pytest

from tests.e2e.utils import run_pls


@pytest.mark.parametrize(
    "args",
    [
        [],
        ["-i", "nerd"],
        ["--icon", "nerd"],
    ],
)
def test_icon_nerd_fonts(args: list[str], icon_workbench: Path):
    args.append(str(icon_workbench))
    proc = run_pls(args)
    for icon in ["", "", ""]:
        assert icon in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["-i", "emoji"],
        ["--icon", "emoji"],
    ],
)
def test_icon_emoji(args: list[str], icon_workbench: Path):
    args.append(str(icon_workbench))
    proc = run_pls(args)
    for icon in ["⏪", "🐳", "📄"]:
        assert icon in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        [],
        ["--align"],
    ],
)
def test_align(args: list[str], align_workbench: Path):
    args.append(str(align_workbench))
    proc = run_pls(args)
    for name in [
        r"\s{2}README.md",
        r"\s{1}.gitignore",
    ]:
        prefix = r"\S\s"  # icon with padding
        line = f"{prefix}{name}"
        assert re.search(line, proc.stdout)


def test_no_align(align_workbench: Path):
    proc = run_pls(["--no-align", str(align_workbench)])
    for name in [
        r"\s{1}README.md",
        r"\s{1}.gitignore",
    ]:
        prefix = r"\S\s"  # icon with padding
        line = f"{prefix}{name}"
        assert re.search(line, proc.stdout)


def test_multi_cols(multi_cols_workbench: Path):
    proc = run_pls(
        ["--multi-cols", str(multi_cols_workbench)],
        env={
            "LINES": "40",
            "COLUMNS": "256",
            "TERM": "xterm-256color",
        },
    )
    assert proc.stdout.count("\n") == 1
    assert proc.stdout.endswith("\n")


@pytest.mark.parametrize(
    "args",
    [
        [],
        ["--no-multi-cols"],
    ],
)
def test_no_multi_cols(args: list[str], multi_cols_workbench: Path):
    args.append(str(multi_cols_workbench))
    proc = run_pls(
        args,
        env={
            "LINES": "40",
            "COLUMNS": "256",
            "TERM": "xterm-256color",
        },
    )
    assert proc.stdout.count("\n") == 3  # contains 3 items
    assert proc.stdout.endswith("\n")


@pytest.mark.parametrize(
    "args",
    [
        [],
        ["-c", "0"],
        ["--collapse", "0"],
    ],
)
def test_no_collapse(args: list[str], collapse_workbench: Path):
    args.insert(0, str(collapse_workbench))
    proc = run_pls(args)
    assert "└─" not in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["-c"],
        ["-c", "1"],
        ["--collapse"],
        ["--collapse", "1"],
    ],
)
def test_collapse_eq_one(args: list[str], collapse_workbench: Path):
    args.insert(0, str(collapse_workbench))
    proc = run_pls(args)
    assert "└─ style.css" in proc.stdout


@pytest.mark.parametrize(
    "args",
    [
        ["-c", "-c"],
        ["-c", "2"],
        ["--collapse", "--collapse"],
        ["--collapse", "2"],
    ],
)
def test_collapse_gt_one(args: list[str], collapse_workbench: Path):
    args.insert(0, str(collapse_workbench))
    proc = run_pls(args)
    assert "style.css" not in proc.stdout
