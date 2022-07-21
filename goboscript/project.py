import gobomatic as gm
from pathlib import Path
from parser import parser
from interpreter import Interpreter
import lark

class TokenException(Exception):
    ...


def build_gm_sprite(sprite_pth: Path) -> gm.Sprite:
    sprite = gm.Sprite(sprite_pth.name, costumes=[""])
    tree = parser.parse(sprite_pth.read_text())
    try:
        Interpreter(sprite, sprite_pth).visit(tree)
    except lark.exceptions.VisitError as e:
        if type(e.orig_exc.args[0]) is lark.Token:
            raise TokenException("name not defined", e.orig_exc.args[0], sprite_pth)
        else:
            raise e.orig_exc
                
    return sprite


def build_gm_project(project_dir: Path) -> gm.Project:
    stage = build_gm_sprite(project_dir / "stage.gs")
    stage.name = "Stage"
    sprites = [stage]
    for sprite in project_dir.glob("*.gs"):
        if sprite.name != "stage.gs":
            sprites.append(build_gm_sprite(sprite))
    project = gm.Project(sprites)
    return project
