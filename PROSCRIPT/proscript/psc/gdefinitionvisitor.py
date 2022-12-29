from pathlib import Path
from typing import cast

from gerror import gTokenError
from gparser import literal
from lark import Token, Tree, Visitor
from lib import file_suggest
from sb3 import gCostume, gSprite


class gDefinitionVisitor(Visitor[Token]):
    def __init__(self, project: Path, sprite: gSprite, tree: Tree[Token]):
        super().__init__()
        self.project = project
        self.sprite = sprite
        self.visit(tree)

    def declr_costumes(self, tree: Tree[Token]):
        for costume in cast(list[Token], tree.children):
            path = self.project / literal(costume)
            if not path.is_file():
                matches = file_suggest(path)
                raise gTokenError(
                    f"File not found {path}",
                    costume,
                    f"Did you mean {matches[0].relative_to(self.project)}?"
                    if matches
                    else None,
                )
            self.sprite.costumes.append(gCostume(path))
