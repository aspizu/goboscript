from pathlib import Path
from sys import argv

from common import CompilerExceptions
from project import compile_project
from fmt import format_file


def build():
    ...


def fmt():
    ...


def main():
    try:
        if argv[1] == "--build":
            compile_project(Path("examples/demo"), Path("examples/demo/build.sb3"))
        elif argv[1] == "--fmt":
            format_file(Path("examples/demo/main.gs"))
    except CompilerExceptions as ex:
        ex.print()
        exit(1)


if __name__ == "__main__":
    main()
