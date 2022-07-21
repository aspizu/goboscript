import project
from pathlib import Path
import fmt as Fmt
from sys import argv
import lark


def build(project_dir: Path, output_pth: Path):
    project.build_gm_project(project_dir).export(output_pth.as_posix())


def fmt(file: Path):
    Fmt.fmt(file)


def fmt_all(pathdir: Path):
    Fmt.fmt_all(pathdir)


if __name__ == "__main__":
    try:
        if argv[1] == "build":
            build(Path(argv[2]), Path(argv[3]))
        elif argv[1] == "fmt":
            fmt(Path(argv[2]))
        elif argv[1] == "fmt-all":
            fmt_all(Path(argv[2]))
    except IndexError:
        print(
            """
ERR: Invalid Command

Usage:
  gsc build path/to/folder path/to/output.sb3
  gsc fmt path/to/file.gs
  gsc fmt path/to/folder
"""[
                1:-1
            ]
        )
    except project.TokenException as e:
        print(f"ERR: {e.args[0]}")
        token: lark.Token = e.args[1]
        file: list[str] = e.args[2].read_text().splitlines()
        print("FILE: " + str(e.args[2]))
        print(f"{token.line-1} | {file[token.line-1]}")
        exit(1)
    except KeyError as e:
        print(f"ERR: undefined $name somewhere: {e}")
