from examples.bench import typ_bench
from examples.utils.main import write_out


def colors():
    with typ_bench() as bench:
        write_out("--det=std", bench=bench, dest_name="on")
        write_out("--det=std", bench=bench, dest_name="off", env={"NO_COLOR": "true"})


if __name__ == "__main__":
    colors()
