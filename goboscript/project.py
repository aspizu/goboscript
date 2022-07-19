from rich import print
import gobomatic as gm
from pathlib import Path
from parser import parser
from interpreter import Interpreter


def DEBUG_print_gm_sprite(sprite: gm.Sprite) -> None:
    print(f"[blue] * Sprite '{sprite.name}':")
    print("[green]  > Costumes:")
    print("      " + "\n      ".join(sprite.costumes))
    print("[green]  > Sounds:")
    print("      " + "\n      ".join(sprite.sounds))
    print("[green]  > Blocks:")
    print("      " + "\n      ".join(map(repr, sprite.blocks)))
    print()


def build_gm_sprite(sprite_pth: Path) -> gm.Sprite:
    print(f"BUILDING SPRITE: '{sprite_pth}'")
    sprite = gm.Sprite(sprite_pth.name, costumes=[""])
    tree = parser.parse(sprite_pth.read_text())
    print("[blue] * Parsed Tree:")
    print(tree)
    Interpreter(sprite, sprite_pth).visit(tree)
    DEBUG_print_gm_sprite(sprite)
    print()
    return sprite


def build_gm_project(project_dir: Path) -> gm.Project:
    sprites = []
    for sprite in project_dir.glob("*.gs"):
        if sprite.name == "stage.gs":
            stage = build_gm_sprite(sprite)
            stage.name = "Stage"
        else:
            sprites.append(build_gm_sprite(sprite))
    project = gm.Project([stage] + sprites)
    return project
