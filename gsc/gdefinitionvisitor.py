from pathlib import Path
from typing import NamedTuple, cast

from gerror import gTokenError
from gparser import literal
from lark import Token, Tree, Visitor
from lib import file_suggest
from sb3 import gCostume, gSprite


class gFunction(NamedTuple):
    warp: bool
    arguments: list[Token]


class gDefinitionVisitor(Visitor[Token]):
    def __init__(self, project: Path, sprite: gSprite, tree: Tree[Token]):
        super().__init__()
        self.project = project
        self.sprite = sprite
        self.functions: dict[Token, gFunction] = {}
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

    def declr_function(self, tree: Tree[Token], warp: bool = True):
        name = cast(Token, tree.children[0])
        if name in self.functions:
            raise gTokenError("Redeclaration of function", name, "Rename this function")
        arguments = cast(list[Token], tree.children[1:-1])
        if arguments == [None]:
            arguments = []
        self.functions[name] = gFunction(warp, arguments)

    def declr_function_nowarp(self, tree: Tree[Token]):
        return self.declr_function(tree, False)
