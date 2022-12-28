from pathlib import Path
from typing import cast

from gerror import gTokenError
from gparser import literal
from lark import Token, Tree, Visitor
from sb3 import gCostume, gSprite


class gDefinitionVisitor(Visitor[Token]):
    def __init__(self, sprite: gSprite, tree: Tree[Token]):
        super().__init__()
        self.sprite = sprite
        self.visit(tree)

    def declr_costumes(self, tree: Tree[Token]):
        for costume in cast(list[Token], tree.children):
            path = Path(literal(costume))
            if not path.is_file():
                raise gTokenError(f"File not found {path}", costume, "remove this line")
            self.sprite.costumes.append(gCostume(path))
