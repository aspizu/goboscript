from pathlib import Path
from gerror import gError, gFileError, wrap_lark_errors
from gparser import gparser
from gspriteinterpreter import gSpriteInterpreter
from lib import EXT, dir_suggest
from sb3 import gProject


def build_gsprite(sprite: Path, globals: list[str], listglobals: list[str]):
    name = sprite.name.removesuffix(f".{EXT}")
    if name == "Stage":
        msg = "`Stage` cannot be used as a sprite name"
        raise gFileError(
            msg,
            "For the stage sprite use the name `stage` (in lowercase)",
            file=sprite,
        )
    name = "Stage" if name == "stage" else name
    ast = wrap_lark_errors(lambda: gparser.parse(sprite.read_text()), sprite)
    return wrap_lark_errors(
        lambda: gSpriteInterpreter(sprite.parent, name, ast, globals, listglobals),
        sprite,
    ).sprite


def build_gproject(project: Path):
    if not project.is_dir():
        matches = dir_suggest(project)
        msg = f"Directory does not exist {project}"
        raise gError(
            msg,
            f"Did you mean {matches[0]}?" if matches else None,
        )
    stage = project / f"stage.{EXT}"
    if not stage.is_file():
        args = f"File does not exist {stage}", f"Create the file {stage}"
        raise gError(*args)
    stage = build_gsprite(stage, [], [])
    globals = list(stage.variables.keys())
    listglobals = list(stage.lists.keys())
    sprites = [
        build_gsprite(sprite, globals, listglobals)
        for sprite in project.glob(f"*.{EXT}")
        if sprite.name != f"stage.{EXT}" and not sprite.name.endswith(f".h.{EXT}")
    ]
    return gProject(stage, sprites)
