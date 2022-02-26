from __future__ import annotations

from pathlib import Path

from pls import globals


def find_configs() -> list[Path]:
    """
    Get the paths for all the relevant ``.pls.yml`` files.

    :return: a list of paths for all ``pls`` config files
    """

    conf_name = ".pls.yml"
    conf_paths = []

    def is_valid(path: Path) -> bool:
        return path.exists() and path.is_file()

    curr_dir: Path = globals.state.directory
    if is_valid(test_path := curr_dir.joinpath(conf_name)):
        conf_paths.append(test_path)

    # Find a config in the current path's ancestors
    conf_paths.extend(
        [
            test_path
            for i in range(globals.state.depth)
            if is_valid(test_path := curr_dir.parents[i].joinpath(conf_name))
        ]
    )

    # Find a config in the Git root.
    if globals.state.git_root is not None:
        test_path = globals.state.git_root.joinpath(conf_name)
        if is_valid(test_path):
            conf_paths.append(test_path)

    # Find a config in the user's home directory.
    try:
        test_path = Path.home().joinpath(conf_name)
        if is_valid(test_path):
            conf_paths.append(test_path)
    except RuntimeError:
        # If home directory cannot be determined, skip it.
        pass

    return conf_paths


conf_files = find_configs()
"""the list of config files applicable to the current directory"""
