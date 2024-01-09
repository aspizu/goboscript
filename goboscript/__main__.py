from __future__ import annotations
import sys
import argparse
from pathlib import Path
from .lib import EXT
from .build import build_gproject
from .error import Error

argparser = argparse.ArgumentParser(
    "gsc",
    description="goboscript is the Scratch compiler.",
    epilog="documentation: https://aspizu.github.io/goboscript-docs",
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
if init_cmd:
    path = Path().absolute()
    file_data = [
        (path / f"stage.{EXT}", 'costumes "blank.svg";\n'),
        (path / f"main.{EXT}", 'costumes "blank.svg";\n\nonflag {\n  say "Hello, World!";\n}\n'),
        (path / "blank.svg", '<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"></svg><!--rotationCenter:0:0-->'),
    ]
    if (path / f"stage.{EXT}").is_file():
        argparser.error("Working directory already contains a goboscript project.")
    ([file_path.open("w").write(content) for file_path, content in file_data])
    sys.exit()
input: Path | None = args.input
if input is None:
    input = Path()
    if not (input / f"stage.{EXT}").is_file():
        argparser.error(
            "Working directory is not a goboscript project, "
            "Please provide proper directory using --input argument."
        )
output: Path | None = args.output
if output is None:
    output = input / f"{input.absolute().stem}.sb3"
    if output.is_dir():
        argparser.error(
            f"{output} is a directory, please provide a different --output argument."
        )
try:
    build_gproject(input).package(output)
except Error as e:
    e.print()
