from __future__ import annotations
from typing import TYPE_CHECKING
from .lib import EXT, dir_suggest
from .sb3 import Project
from .error import Error, FileError, wrap_lark_errors
from .paste import PasteBuilder
from .spriteinterpreter import SpriteInterpreter

if TYPE_CHECKING:
    from pathlib import Path
    from lark import Lark


def build_gsprite(
    sprite: Path, globals: list[str], listglobals: list[str], parser: Lark
):
    name = sprite.name.removesuffix(f".{EXT}")
    if name == "Stage":
        msg = "`Stage` cannot be used as a sprite name"
        raise FileError(
            msg,
            "For the stage sprite use the name `stage` (in lowercase)",
            file=sprite,
        )
    name = "Stage" if name == "stage" else name
    paste = PasteBuilder(relative=sprite.parent).include(sprite).paste
    ast = wrap_lark_errors(lambda: parser.parse("".join(paste.lines)), paste, sprite)
    return wrap_lark_errors(
        lambda: SpriteInterpreter(
            sprite.parent, name, ast, globals, listglobals, parser
        ),
        paste,
        sprite,
    ).sprite


def build_gproject(project: Path, parser: Lark):
    if not project.is_dir():
        matches = dir_suggest(project)
        msg = f"Directory does not exist {project}"
        raise Error(
            msg,
            f"Did you mean {matches[0]}?" if matches else None,
        )
    stage = project / f"stage.{EXT}"
    if not stage.is_file():
        args = f"File does not exist {stage}", f"Create the file {stage}"
        raise Error(*args)
    stage = build_gsprite(stage, [], [], parser)
    globals = list(stage.variables.keys())
    listglobals = list(stage.lists.keys())
    sprites = [
        build_gsprite(sprite, globals, listglobals, parser)
        for sprite in project.glob(f"*.{EXT}")
        if sprite.name != f"stage.{EXT}" and not sprite.name.endswith(f".h.{EXT}")
    ]
    return Project(stage, sprites)
