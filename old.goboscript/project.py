from rich import print
from os.path import basename
import parser
import gobomatic as gm
from glob import glob


def sprite_from_file(file: str, project_root: str, name: str = None) -> gm.Sprite:
    sprite = gm.Sprite(name or basename(file)[:-3], [""])
    with open(file, "r") as fp:
        file = fp.read()
    tree = parser.parse(file)
    print(tree)
    first_pass = parser.FirstPass(sprite, project_root)
    first_pass.visit(tree)

    second_pass = parser.SecondPass(
        sprite, first_pass.vars, first_pass.lsts, first_pass.funcs
    )
    blocks = second_pass.transform(tree)
    print(sprite.costumes)
    print(sprite.variables)
    print(sprite.lists)
    print(first_pass.funcs)
    print(blocks)
    sprite.blocks.extend(blocks)
    return sprite


def build_project(folder: str, out: str):
    sprites = []
    for sprite in glob(f"{folder}/*.gs"):
        if basename(sprite) == "stage.gs":
            stage = sprite_from_file(sprite, folder, name="Stage")
        else:
            sprites.append(sprite_from_file(sprite, folder))
    gm.Project([stage] + sprites).export(out)
