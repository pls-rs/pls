from __future__ import annotations

from pathlib import Path

import pytest

from tests.e2e.utils import run_pls


@pytest.mark.parametrize(
    "workbench_idx, present_files, absent_files",
    [
        (0, ["a_1", "a_2"], ["b_1", "b_2"]),
        (1, ["b_1", "b_2"], ["a_1", "a_2"]),
    ],
)
def test_single_working_directory(
    workbench_idx: int,
    present_files: list[str],
    absent_files: list[str],
    pos_workbenches: tuple[Path, Path],
):
    path = pos_workbenches[workbench_idx]
    proc = run_pls([str(path.resolve())])
    for file_name in present_files:
        assert file_name in proc.stdout
    for file_name in absent_files:
        assert file_name not in proc.stdout


def test_multiple_working_directory(pos_workbenches: tuple[Path, Path]):
    paths = [str(path.resolve()) for path in pos_workbenches]
    proc = run_pls(paths)

    for dir_name in ["a", "b"]:
        assert f"{dir_name}:" in proc.stdout
        for index in range(1, 3):
            assert f"{dir_name}_{index}" in proc.stdout
