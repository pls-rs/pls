import os

from examples.bench import typ_bench
from examples.utils.fs import fs, mksock
from examples.utils.main import write_out, copy_write_conf


def suffixes():
    with typ_bench() as bench:
        write_out(bench=bench, dest_name="on")
        write_out("--suffix=false", bench=bench, dest_name="off")
        copy_write_conf(bench)
        write_out(bench=bench, dest_name="confd")


def icons():
    with fs(
        (
            "icons",
            [
                ("dir", []),
                ("sym", lambda p: p.symlink_to("./dir")),
                ".pls.yml",
                ".gitignore",
                "README.md",
                ("fifo", lambda p: os.mkfifo(p)),
                ("socket", lambda p: mksock(p)),
            ],
        )
    ) as bench:
        write_out(bench=bench, dest_name="on")
        write_out("--icon=false", bench=bench, dest_name="off")
        copy_write_conf(bench)
        write_out(bench=bench, dest_name="confd")


def symlinks():
    with fs(
        (
            "symlink",
            [
                ("dir", []),
                ("a", lambda p: p.symlink_to("a")),
                ("b", lambda p: p.symlink_to("c")),
                ("c", lambda p: p.symlink_to("b")),
                ("d", lambda p: p.symlink_to("nonexistent")),
                ("e", lambda p: p.symlink_to("dir")),
                ("f", lambda p: p.symlink_to("dir")),
                ("g", lambda p: p.symlink_to("f")),
            ],
        )
    ) as bench:
        # Make the symlink unreadable.
        try:
            os.chmod(bench / "e", 000, follow_symlinks=False)
        except NotImplementedError:
            pass

        write_out(bench=bench, dest_name="on")
        write_out("--sym=false", bench=bench, dest_name="off")
        copy_write_conf(bench)
        write_out(bench=bench, dest_name="confd")

        # Re-allow deletion during cleanup.
        try:
            os.chmod(bench / "e", 777, follow_symlinks=False)
        except NotImplementedError:
            pass


def alignment():
    with fs(
        (
            "alignment",
            [
                ".pls.yml",
                ".gitignore",
                "README.md",
                "LICENSE",
            ],
        )
    ) as bench:
        write_out(bench=bench, dest_name="on")
        write_out("--align=false", bench=bench, dest_name="off")


if __name__ == "__main__":
    suffixes()
    icons()
    symlinks()
    alignment()
