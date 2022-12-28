from typing import cast

from gblocktransformer import gBlockTransformer
from gdefinitionvisitor import gDefinitionVisitor
from lark import Token, Tree
from lark.visitors import Interpreter
from sb3 import gHatBlock, gSprite


class gSpriteInterpreter(Interpreter[Token, None]):
    def __init__(self, name: str, tree: Tree[Token]):
        super().__init__()
        self.sprite = gSprite(name, [], [], [], [])
        self.gdefinitionvisitor = gDefinitionVisitor(self.sprite, tree)
        if len(self.sprite.costumes) == 0:
            ...
        self.visit(tree)

    def declr_hat(self, tree: Tree[Token]):
        self.sprite.blocks.append(cast(gHatBlock, gBlockTransformer().transform(tree)))
