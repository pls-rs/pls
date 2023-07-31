"""
This module contains reusable benches that are used by multiple example
generation files.
"""

import os
import uuid

from examples.utils.fs import mksock, mkbigfile, fs


def typ_bench(name: str = None):
    if name is None:
        name = uuid.uuid4().hex

    return fs(
        (
            name,
            [
                ("dir", []),
                ("sym", lambda p: p.symlink_to("./dir")),
                ("fifo", lambda p: os.mkfifo(p)),
                ("socket", lambda p: mksock(p)),
                ("char_dev", lambda p: p.symlink_to("/dev/null")),
                ("block_dev", lambda p: p.symlink_to("/dev/disk0")),
                ("file", lambda p: mkbigfile(p, size=1024**2)),
            ],
        )
    )


def grid_bench(name: str = None):
    if name is None:
        name = uuid.uuid4().hex

    return fs(
        (
            name,
            [
                "file_abcd",
                "file_efgh",
                "file_ijkl",
                (".file_mnop", lambda p: p.symlink_to("file_abcd")),
            ],
        )
    )


def ts_bench(name: str = None):
    if name is None:
        name = uuid.uuid4().hex

    return fs(
        (
            name,
            [
                (
                    "src",
                    [
                        "index.ts",
                        "index.js",
                        "lib.ts",
                        "lib.js",
                        "no_parent.js",
                        "no_child.ts",
                    ],
                ),
                "package.json",
                "pnpm-lock.yaml",
                ".gitignore",
                ".prettierignore",
                "prettier.config.js",
                "tsconfig.json",
                "README.md",
                "justfile",
            ],
        )
    )
