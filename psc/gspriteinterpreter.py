from pathlib import Path
from typing import cast

from gblocktransformer import gBlockTransformer
from gdefinitionvisitor import gDefinitionVisitor
from gerror import gFileError
from lark import Token, Tree
from lark.visitors import Interpreter
from sb3 import gHatBlock, gSprite


class gSpriteInterpreter(Interpreter[Token, None]):
    def __init__(self, project: Path, name: str, tree: Tree[Token]):
        super().__init__()
        self.sprite = gSprite(name, [], [], [], [])
        self.gdefinitionvisitor = gDefinitionVisitor(project, self.sprite, tree)
        self.gblocktransformer = gBlockTransformer(self.gdefinitionvisitor)
        if len(self.sprite.costumes) == 0:
            raise gFileError("No costumes defined", "Add a costumes statement")
        self.visit(tree)

    def declr_hat(self, tree: Tree[Token]):
        self.sprite.blocks.append(
            cast(gHatBlock, self.gblocktransformer.transform(tree))
        )
