from pathlib import Path

import gobomatic as gm
import lark
from rich import print as rprint

from common import CompilerError, CompilerFileError, parser
from compiler import Interpreter


def compile_sprite(sprite_pth: Path, name: str = None) -> gm.Sprite:
    try:
        sprite = gm.Sprite(name or sprite_pth.name, [""])
        tree = parser.parse(sprite_pth.read_text())
        rprint(tree)
        interpreter = Interpreter(sprite_pth, sprite)
        interpreter.visit(tree)
    except lark.exceptions.VisitError as ex:
        if isinstance(ex.orig_exc, CompilerFileError):
            ex.orig_exc.file_pth = sprite_pth
        raise ex.orig_exc
    except lark.exceptions.UnexpectedCharacters as ex:
        token = lark.Token("", "*")
        token.line = ex.line
        token.column = ex.column
        new_ex = CompilerFileError(token, "Unexpected characters")
        new_ex.file_pth = sprite_pth
        raise new_ex
    except CompilerFileError as ex:
        ex.file_pth = sprite_pth
        raise ex
    return sprite


def compile_project(project_dir: Path, output_pth: Path) -> None:
    if not (project_dir / "stage.gs").is_file():
        raise CompilerError(
            msg="Project does not have a stage.gs file", tip="Create a stage.gs file"
        )
    sprites = [compile_sprite(project_dir / "stage.gs", name="Stage")] + [
        compile_sprite(sprite_pth)
        for sprite_pth in project_dir.glob("*.gs")
        if sprite_pth.name != "stage.gs"
    ]
    gm.Project(sprites).export(output_pth.absolute().as_posix())
