from pathlib import Path

from lark.exceptions import VisitError
from sb3 import *

from .gexception import *
from .gparser import gparser
from .gsprite_interpreter import gSpriteInterpreter


def build_gsprite(sprite_pth: Path, prefix: Path) -> gSprite:
    tree = gparser.parse(sprite_pth.read_text())
    try:
        inter = gSpriteInterpreter(tree, prefix)
        return inter.to_gSprite(sprite_pth.name)
    except VisitError as e:
        if isinstance(e.orig_exc, gCodeError):
            e.orig_exc.print(sprite_pth)
            exit(1)
        else:
            raise e.orig_exc
    except gCodeError as e:
        e.print(sprite_pth)
        exit(1)


def build_gproject(project_dir: Path) -> gProject:
    prefix = project_dir
    if not (project_dir / "stage.gs").is_file():
        raise gError("Project does not have stage.gs")
    return gProject(
        build_gsprite(project_dir / "stage.gs", prefix),
        [
            build_gsprite(i, prefix)
            for i in project_dir.glob("*.gs")
            if i.name != "stage.gs"
        ],
    )
