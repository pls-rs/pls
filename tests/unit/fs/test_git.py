from pathlib import Path
from unittest.mock import Mock, patch

from pls.fs.git import get_git_statuses


def test_statuses_combines_two_git_cmds():
    common_entries = [
        " A not_updated",
        "MM updated_index_updated_worktree",
        "D  deleted_index",
        " R renamed_worktree -> new_worktree",
    ]
    # git status --porcelain --untracked-files --ignored
    u_i = Mock(
        stdout="\n".join(
            [
                *common_entries,
                "!! ignored_dir/ignored_file",
                "?? untracked_dir/untracked_file",
            ]
        )
    )
    # git status --porcelain --untracked-files=normal --ignored=matching
    u_normal_i_matching = Mock(
        stdout="\n".join(
            [
                *common_entries,
                "!! ignored_dir/",
                "?? untracked_dir/",
            ]
        )
    )
    exec_git = Mock(side_effect=[u_i, u_normal_i_matching])

    with patch("pls.fs.git.exec_git", exec_git):
        statuses = get_git_statuses(Path("."))

    assert len(statuses) == 8
    assert statuses == {
        Path("not_updated"): " A",
        Path("updated_index_updated_worktree"): "MM",
        Path("deleted_index"): "D ",
        Path("new_worktree"): " R",
        Path("ignored_dir/ignored_file"): "!!",
        Path("ignored_dir/"): "!!",
        Path("untracked_dir/untracked_file"): "??",
        Path("untracked_dir/"): "??",
    }
