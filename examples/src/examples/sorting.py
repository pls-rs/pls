from examples.utils.main import write_out
from examples.utils.fs import mkbigfile, fs


def sorting():
    with fs(
        (
            "sorting",
            [
                ("dir_a", []),
                ("file_b.txt", lambda p: mkbigfile(p, size=1024**1)),
                ("dir_c", []),
                ("file_d.txt", lambda p: mkbigfile(p, size=1024**2)),
                ("dir_e", []),
                ("file_f.txt", lambda p: mkbigfile(p, size=1024**0)),
            ],
        )
    ) as bench:
        write_out(bench=bench, dest_name="def")
        write_out(
            "--det=ino",
            "--det=typ",
            "--det=size",
            "--sort=cat_",
            "--sort=size_",
            "--sort=ino",
            bench=bench,
            dest_name="cust",
        )


if __name__ == "__main__":
    sorting()
