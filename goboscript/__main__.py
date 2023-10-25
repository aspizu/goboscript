from __future__ import annotations
import sys
import argparse
from pathlib import Path
from .lib import EXT, Watcher
from .build import build_gproject
from .error import Error
from .parser import get_parser

argparser = argparse.ArgumentParser(
    "gsc",
    description="goboscript compiler",
    epilog="https://github.com/aspizu/goboscript",
)


def input_t(argument: str) -> Path:
    path = Path(argument)
    if not path.is_dir():
        raise argparse.ArgumentTypeError(f"{path} is not a directory.")
    if not (path / f"stage.{EXT}").is_file():
        raise argparse.ArgumentTypeError(
            f"{path}/stage.{EXT} not found. Is this a goboscript project?"
        )
    return path


def output_t(argument: str) -> Path:
    path = Path(argument)
    if path.is_dir():
        raise argparse.ArgumentTypeError(f"{path} is a directory.")
    return path


argparser.add_argument(
    "--init",
    action="store_true",
    help="Create a new goboscript project. (Exits after writing the template.)",
)
argparser.add_argument(
    "--watch", action="store_true", help="Watch for file changes and recompile."
)
argparser.add_argument(
    "--semi", action="store_true", help="Use old semi-colon based syntax."
)
argparser.add_argument(
    "-input",
    type=input_t,
    help="Project directory. (If not given, working directory is chosen.)",
    default=None,
)
argparser.add_argument(
    "-output",
    type=output_t,
    help="Path to output (.sb3) file. (If not given, output file will be inside input directory with same name.)",
    default=None,
)
args = argparser.parse_args()
init_cmd = args.init
watch = args.watch
semi = args.semi
if semi:
    parser = get_parser(semi=True)
else:
    parser = get_parser(semi=False)
if init_cmd:
    path = Path().absolute()
    if (path / f"stage.{EXT}").is_file():
        argparser.error("Working directory already contains a goboscript project.")
    (path / f"stage.{EXT}").open("w").write('costumes "blank.svg";\n')
    (path / f"main.{EXT}").open("w").write(
        'costumes "blank.svg";\n\n' + "onflag {\n" '  say "Hello, World!";\n' "}\n"
    )
    (path / "blank.svg").open("w").write(
        '<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"></svg><!--rotationCenter:0:0-->'
    )
    sys.exit()
input: Path | None = args.input
if input is None:
    input = Path()
    if not (input / f"stage.{EXT}").is_file():
        argparser.error(
            "Working directory is not a goboscript project, "
            "please provide an --input argument."
        )
output: Path | None = args.output
if output is None:
    output = input / f"{input.absolute().stem}.sb3"
    if output.is_dir():
        argparser.error(
            f"{output} is a directory, please provide a different --output argument."
        )


class ProjectWatcher(Watcher):
    input: Path
    output: Path

    def on_change(self, file: Path) -> None:  # noqa: ARG002
        try:
            build_gproject(self.input, parser).package(self.output)
        except Error as err:
            err.print()


if watch:
    watcher = ProjectWatcher(list(input.glob(f"*.{EXT}")))
    watcher.input = input
    watcher.output = output
    watcher.watch()
else:
    try:
        build_gproject(input, parser).package(output)
    except Error as err:
        err.print()
