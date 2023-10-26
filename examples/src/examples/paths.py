from pathlib import Path

from examples.utils.fs import fs
from examples.utils.main import write_out, copy_write_conf

PROJECT_ROOT = Path(__file__).parents[3]


def files_and_dirs():
    write_out(
        "README.md",
        "Cargo.toml",
        "Cargo.lock",
        "src",
        "docs",
        bench=PROJECT_ROOT,
        include_bench=False,
        dest_name="files_and_dirs",
    )


def file_group():
    with fs(("file_group", ["a", ("subdir", ["a"])])) as bench:
        copy_write_conf(bench, "outer")
        copy_write_conf(bench / "subdir", "inner")
        write_out(
            "a",
            "./../file_group/./subdir/a",
            "--det=std",
            cwd=bench,
            include_bench=False,
            bench=bench,
            dest_name="file_group",
        )


def symlinks():
    with fs(
        (
            "symlinks",
            [
                ("dir", ["README.md", "LICENSE"]),
                ("sym", lambda p: p.symlink_to("./dir")),
            ],
        )
    ) as bench:
        write_out(
            "sym",
            cwd=bench,
            include_bench=False,
            bench=bench,
            dest_name="symlinks",
        )
        write_out(
            "./dir",
            cwd=bench,
            include_bench=False,
            bench=bench,
            dest_name="destination",
        )


if __name__ == "__main__":
    files_and_dirs()
    file_group()
    symlinks()
