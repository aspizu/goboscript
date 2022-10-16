from parser import parser
from pathlib import Path

from lark.exceptions import VisitError
from rich import print

from gsprite_interpreter import gSpriteInterpreter
from scratch import *
from utils import gTokenException

PWD = Path()


def build_gsprite(file: Path) -> gSprite:
    tree = parser.parse(file.read_text())
    print(tree)
    inter = gSpriteInterpreter(file.parent)
    try:
        inter.interpret(tree)
    except VisitError as e:
        e = e.orig_exc
        if isinstance(e, gTokenException):
            e.print(file)
            exit(1)
        else:
            raise e
    except gTokenException as e:
        e.print(file)
        exit(1)
    return inter.to_gsprite(
        "Stage" if file.name == "stage.gs" else file.name[:-3].capitalize()
    )


def build_gproject(proj_dir: Path) -> gProject:
    sprites: list[gSprite] = []
    for i in proj_dir.glob("*.gs"):
        if i.name != "stage.gs":
            sprites.append(build_gsprite(i))
    return gProject(build_gsprite(proj_dir / "stage.gs"), tuple(sprites))


def build(proj_dir: Path, package_pth: Path) -> None:
    build_gproject(proj_dir).package(package_pth)


build(PWD / "examples" / "demo", PWD / "package.sb3")
