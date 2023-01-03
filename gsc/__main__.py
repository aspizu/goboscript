import sys
from pathlib import Path

from gbuild import build_gproject
from gerror import gError


def parse_args():
    if len(sys.argv) != 3:
        raise gError(
            "Expected 2 arguments",
            "gsc <project dir path> <output sb3 path>",
        )
    project = Path(sys.argv[1])
    output = Path(sys.argv[2])
    build_gproject(project).package(output)


def main():
    try:
        parse_args()
    except gError as e:
        e.print()
        exit(1)


if __name__ == "__main__":
    main()
