from pathlib import Path
from typing import NamedTuple, cast

from gerror import gTokenError
from gparser import literal
from lark import Token, Tree, Visitor
from lib import file_suggest
from sb3 import gCostume, gList, gSprite, gVariable


class gFunction(NamedTuple):
    warp: bool
    arguments: list[Token]


class gMacro(NamedTuple):
    arguments: list[str]
    body: Tree[Token]


class gDefinitionVisitor(Visitor[Token]):
    def __init__(self, project: Path, sprite: gSprite, tree: Tree[Token]):
        super().__init__()
        self.project = project
        self.sprite = sprite
        self.macros: dict[str, gMacro] = {}
        self.functions: dict[Token, gFunction] = {}
        self.globals: list[Token] = []
        self.listglobals: list[Token] = []
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

    def declr_macro(self, tree: Tree[Token]):
        name = cast(Token, tree.children[0])
        if name in self.macros:
            raise gTokenError("Redeclaration of macro", name, "Rename this macro")
        arguments = cast(list[Token], tree.children[1:-1])
        if arguments == [None]:
            arguments = []
        self.macros[name] = gMacro(
            [str(i) for i in arguments], cast(Tree[Token], tree.children[-1])
        )

    def declr_function_nowarp(self, tree: Tree[Token]):
        return self.declr_function(tree, False)

    def varset(self, tree: Tree[Token]):
        if tree.children[0] in self.globals:
            return
        if gVariable(tree.children[0]) not in self.sprite.variables:
            self.sprite.variables.append(gVariable(tree.children[0]))

    def listset(self, tree: Tree[Token]):
        if tree.children[0] in self.listglobals:
            return
        if gList(tree.children[0]) not in self.sprite.lists:
            self.sprite.lists.append(gList(tree.children[0]))

    def declr_globals(self, tree: Tree[Token]):
        for variable in cast(list[Token], tree.children):
            if variable in self.globals:
                raise gTokenError(
                    f"variable `{variable}` was repeated", variable, "Remove this"
                )
            self.globals.append(variable)
            try:
                self.sprite.variables.remove(gVariable(variable))
            except ValueError:
                pass

    def declr_listglobals(self, tree: Tree[Token]):
        for lst in cast(list[Token], tree.children):
            if lst in self.listglobals:
                raise gTokenError(f"list `{lst}` was repeated", lst, "Remove this")
            self.listglobals.append(lst)
            try:
                self.sprite.lists.remove(gList(lst))
            except ValueError:
                pass
