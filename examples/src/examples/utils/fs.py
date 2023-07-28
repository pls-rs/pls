import os
import shutil
import socket
from collections.abc import Generator
from contextlib import contextmanager
from pathlib import Path
from typing import Callable


Creator = Callable[[Path], None]
FsStructure = str | tuple[str, Creator] | tuple[str, list["FsStructure"]]


WORKBENCHES = Path("/") / "tmp" / "workbenches"


def mkbigfile(path: Path, content: str = "Hello, World!", size: int | None = None):
    """
    Create a big file with the given content and size.

    The size is used as an offset to write the data, making a sparse file, that
    has the final size equal to the sum of ``size`` and the length of
    ``content``.

    This function is designed to mimic the signature of OS functions like
    ``os.mkdir`` and ``os.mkfifo``.

    :param path: the path where to create the file
    :param content: the content to write in the file
    :param size: the base size of the file before including the content length
    """

    with path.open("w") as file:
        if size is not None:
            file.seek(size)
        file.write(content)


def mksock(path: Path):
    """
    Make a socket file at the given path.

    This function is designed to mimic the signature of OS functions like
    ``os.mkdir`` and ``os.mkfifo``.

    :param path: the path where to create the socket file
    """

    sock = socket.socket(socket.AF_UNIX)
    sock.bind(str(path.absolute()))


@contextmanager
def fs(
    structure: FsStructure,
    workdir: Path = WORKBENCHES,
) -> Generator[Path, None, None]:
    """
    Given a structure of files and folders, convert them to an actual path on
    the file system and yield a reference to the root path of the structure.
    Once the context is closed, destroy the nodes from the file system.

    This function must be used as a context manager using ``with``...``as``.

    :param structure: the node hierarchy depicted using lists and tuples
    :param workdir: the directory in which all operations are taking place
    :yield: the path of the top created node
    """

    path = _create_fs(structure, workdir)
    try:
        yield path
    finally:
        _destroy_fs(path)


def _create_fs(structure: FsStructure, workdir: Path = WORKBENCHES) -> Path:
    """
    Given a structure of files and folders, convert them to an actual path on
    the file system. This function recursively invokes itself to create nested
    paths.

    :param structure: the structure of the nodes, using lists and tuples
    :param workdir: the directory in which all operations are taking place
    :return: the path of the top created node
    """

    if not workdir.exists():
        workdir.mkdir(mode=0o755)

    if isinstance(structure, str):
        node = workdir.joinpath(structure)
        node.touch(mode=0o644)  # file
    else:  # isinstance(structure, tuple):
        (name, other) = structure
        if callable(other):  # `other` is a custom creator function.
            node = workdir.joinpath(name)
            other(node)  # custom creator
        else:  # isinstance(other, list):  # `other` is a list of children.
            node = workdir.joinpath(name)
            node.mkdir(mode=0o755)  # directory
            for child in other:
                _create_fs(child, node)
    return node


def _destroy_fs(path: Path):
    """
    Destroy the given path, and all its contents if it is a directory. If the
    path does not exist in the first place, it does nothing.

    :param path: the path to get rid of
    """

    if not path.exists():
        return

    if path.is_dir():
        shutil.rmtree(path)
    else:
        os.remove(path)
