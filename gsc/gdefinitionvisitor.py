from pathlib import Path
from typing import NamedTuple, cast

from gerror import gTokenError
from gparser import literal
from lark.lexer import Token
from lark.tree import Tree
from lark.visitors import Interpreter, Visitor
from lib import file_suggest
from sb3 import gCostume, gList, gSprite, gVariable


class gFunction(NamedTuple):
    name: Token
    warp: bool
    arguments: list[Token]
    locals: list[str]


class gMacro(NamedTuple):
    arguments: list[str]
    body: Tree[Token]


class LocalsCollector(Visitor[Token]):
    def __init__(
        self,
        tree: Tree[Token],
        sprite: gSprite,
        globals: list[Token],
        listglobals: list[Token],
        name: str | None = None,
    ):
        super().__init__()
        self.sprite = sprite
        self.globals = globals
        self.listglobals = listglobals
        self.locals: list[str] = []
        self.name = name
        self.visit(tree)

    def localvar(self, tree: Tree[Token]):
        if not self.name:
            return
        self.locals.append(str(tree.children[0]))
        self.sprite.variables.append(gVariable(self.name + "." + str(tree.children[0])))

    def varset(self, tree: Tree[Token]):
        if tree.children[0] in self.locals:
            return
        if tree.children[0] in self.globals:
            return
        if gVariable(tree.children[0]) not in self.sprite.variables:
            self.sprite.variables.append(gVariable(tree.children[0]))

    def listset(self, tree: Tree[Token]):
        if tree.children[0] in self.listglobals:
            return
        if gList(cast(Token, tree.children[0])) not in self.sprite.lists:
            self.sprite.lists.append(gList(cast(Token, tree.children[0])))


class gDefinitionVisitor(Interpreter[Token, None]):
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
            pattern = literal(costume)
            if "*" in pattern:
                paths = sorted(self.project.glob(pattern), key=lambda path: path.stem)
                if len(paths) == 0:
                    raise gTokenError(
                        f"Glob does not match any files {pattern}", costume
                    )
                for pattern in paths:
                    self.sprite.costumes.append(gCostume(pattern))
            else:
                path = self.project / pattern
                if not path.is_file():
                    matches = file_suggest(path)
                    raise gTokenError(
                        f"Costume file not found {pattern}",
                        costume,
                        (
                            f"Did you mean {matches[0].relative_to(self.project)}?"
                            if matches
                            else None
                        ),
                    )
                self.sprite.costumes.append(gCostume(path))

    def datalist(self, tree: Tree[Token]) -> None:
        name = cast(Token, tree.children[0])
        file: Path = self.project / literal(cast(Token, tree.children[1]))
        self.sprite.lists.append(gList(name, file.read_text().splitlines()))

    def declr_function(self, tree: Tree[Token], warp: bool = True):
        name = cast(Token, tree.children[0])
        if name in self.functions:
            raise gTokenError("Redeclaration of function", name, "Rename this function")
        arguments: list[Token] = []
        for argument in cast(list[Token | None], tree.children[1:-1]):
            if argument is None:
                break
            if argument in arguments:
                raise gTokenError(
                    f"Argument `{argument}` was repeated",
                    argument,
                    "Rename this argument",
                )
            arguments.append(argument)
        locals = LocalsCollector(
            cast(Tree[Token], tree.children[-1]),
            self.sprite,
            self.globals,
            self.listglobals,
            name,
        ).locals
        self.functions[name] = gFunction(name, warp, arguments, locals)

    def declr_on(self, tree: Tree[Token]):
        LocalsCollector(
            cast(Tree[Token], tree.children[-1]),
            self.sprite,
            self.globals,
            self.listglobals,
        )

    declr_function_nowarp = declr_function
    declr_onflag = declr_on
    declr_onclone = declr_on
    declr_ontimer = declr_on
    declr_onloudness = declr_on
    declr_onkey = declr_on
    declr_onbackdrop = declr_on

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

    def declr_comment(self, tree: Tree[Token]):
        comment: str = literal(cast(Token, tree.children[0]))
        self.sprite.comment = comment

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
