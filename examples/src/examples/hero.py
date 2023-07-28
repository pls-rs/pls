from pathlib import Path

from examples.utils.main import write_out


PROJECT_ROOT = Path(__file__).parents[3]


def hero():
    write_out("--det=all", bench=PROJECT_ROOT, dest_name="hero")


if __name__ == "__main__":
    hero()
