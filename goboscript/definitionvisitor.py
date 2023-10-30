from __future__ import annotations
from typing import TYPE_CHECKING, NamedTuple, cast
from lark.tree import Tree
from lark.lexer import Token
from lark.visitors import Visitor, Interpreter
from .lib import tok, file_suggest
from .sb3 import List, Sprite, Costume, Variable
from .error import RangeError
from .parser import literal

if TYPE_CHECKING:
    from pathlib import Path


class Function(NamedTuple):
    name: Token
    warp: bool
    arguments: list[Token]
    locals: list[str]
    proccode: str | None = None


class Macro(NamedTuple):
    arguments: list[str]
    body: Tree[Token]


class BlockMacro(NamedTuple):
    arguments: list[str]
    body: Tree[Token]


class LocalsCollector(Visitor[Token]):
    def __init__(
        self,
        tree: Tree[Token],
        sprite: Sprite,
        globals: list[str],
        listglobals: list[str],
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
        if self.name is None:
            return
        token = cast(Token, tree.children[0])
        self.locals.append(str(token))
        qualname = f"{self.name}:{token}"
        self.sprite.variables[qualname] = Variable(qualname, token)

    def add_variable(self, token: Token):
        qualname = str(token)
        if qualname in self.globals:
            return
        if qualname in self.locals:
            return
        if qualname in self.sprite.variables:
            return
        if qualname in self.sprite.lists:
            return
        self.sprite.variables[qualname] = Variable(qualname, token)

    def add_list(self, token: Token):
        qualname = str(token)
        if qualname in self.listglobals:
            return
        if qualname in self.sprite.lists:
            return
        if qualname in self.sprite.variables:
            return
        self.sprite.lists[qualname] = List(token)

    def varset(self, tree: Tree[Token]):
        token = cast(Token, tree.children[0])
        self.add_variable(token)

    def listset(self, tree: Tree[Token]):
        token = cast(Token, tree.children[0])
        self.add_list(token)

    def declr_variables(self, tree: Tree[Token]):
        for token in tree.children:
            self.add_variable(cast(Token, token))

    def declr_lists(self, tree: Tree[Token]):
        for token in tree.children:
            self.add_list(cast(Token, token))


class DefinitionVisitor(Interpreter[Token, None]):
    def __init__(
        self,
        project: Path,
        sprite: Sprite,
        tree: Tree[Token],
        globals: list[str],
        listglobals: list[str],
    ):
        super().__init__()
        self.project = project
        self.sprite = sprite
        self.macros: dict[Token, Macro] = {}
        self.block_macros: dict[Token, BlockMacro] = {}
        self.functions: dict[str, Function] = {
            # Scratch Addons Debugger blocks
            "breakpoint": Function(
                tok("breakpoint"),
                False,
                [],
                [],
                proccode="\u200B\u200Bbreakpoint\u200B\u200B",
            ),
            "log": Function(
                tok("log"),
                False,
                [tok("message")],
                [],
                proccode="\u200B\u200Blog\u200B\u200B %s",
            ),
            "warn": Function(
                tok("warn"),
                False,
                [tok("message")],
                [],
                proccode="\u200B\u200Bwarn\u200B\u200B %s",
            ),
            "error": Function(
                tok("error"),
                False,
                [tok("message")],
                [],
                proccode="\u200B\u200Berror\u200B\u200B %s",
            ),
        }
        self.globals: list[str] = globals
        self.listglobals: list[str] = listglobals
        self.visit(tree)

    def declr_costumes(self, tree: Tree[Token]):
        for costume in cast(list[Token], tree.children):
            pattern = literal(costume)
            if pattern == "*machine:ASCII":
                for i in range(33, 127):
                    self.sprite.costumes.append(
                        Costume(
                            self.project / "blank.svg",
                            alias=chr(i),
                        )
                    )
            elif "*" in pattern:
                paths = sorted(self.project.glob(pattern), key=lambda path: path.stem)
                if len(paths) == 0:
                    raise RangeError(
                        costume, f"Glob does not match any files {pattern}"
                    )
                for pattern in paths:
                    self.sprite.costumes.append(Costume(pattern))
            else:
                path = self.project / pattern
                if not path.is_file():
                    matches = file_suggest(path)
                    raise RangeError(
                        costume,
                        f"Costume file not found {pattern}",
                        (
                            f"Did you mean {matches[0].relative_to(self.project)}?"
                            if matches
                            else None
                        ),
                    )
                self.sprite.costumes.append(Costume(path))

    def datalist(self, tree: Tree[Token]) -> None:
        token = cast(Token, tree.children[0])
        qualname = str(token)
        path = cast(Token, tree.children[1])
        file: Path = self.project / literal(path)
        if not file.is_file():
            matches = file_suggest(file)
            raise RangeError(
                path,
                "Data file not found.",
                f"Did you mean {matches[0].relative_to(self.project)}?"
                if matches
                else None,
            )

        self.sprite.lists[qualname] = List(token, file.read_text().splitlines())

    def imagelist(self, tree: Tree[Token]) -> None:
        from PIL import Image

        token = cast(Token, tree.children[0])
        qualname = str(token)
        path = cast(Token, tree.children[1])
        format = cast(Token | None, tree.children[2])
        file: Path = self.project / literal(path)
        if not file.is_file():
            matches = file_suggest(file)
            raise RangeError(
                path,
                "Data file not found.",
                f"Did you mean {matches[0].relative_to(self.project)}?"
                if matches
                else None,
            )

        image = Image.open(file)
        if format is None:
            data = list(image.tobytes())
        else:
            raise RangeError(
                format,
                "Invalid imagelist format.",
                "Formats are not implemented yet, open a issue at https://github/aspizu/goboscript/issues",
            )
        self.sprite.lists[qualname] = List(token, list(map(str, data)))

    def declr_function(self, tree: Tree[Token], *, warp: bool = True):
        name = cast(Token, tree.children[0])
        if name in self.functions:
            raise RangeError(name, "Redeclaration of function", "Rename this function")
        arguments: list[Token] = []
        for argument in cast(list[Token | None], tree.children[1:-1]):
            if argument is None:
                break
            if argument in arguments:
                raise RangeError(
                    argument,
                    f"Argument `{argument}` was repeated",
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
        self.functions[name] = Function(name, warp, arguments, locals)

    def declr_variables(self, tree: Tree[Token]):
        LocalsCollector(tree, self.sprite, self.globals, self.listglobals)

    def declr_lists(self, tree: Tree[Token]):
        LocalsCollector(tree, self.sprite, self.globals, self.listglobals)

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
            raise RangeError(name, "Redeclaration of macro", "Rename this macro")
        arguments = cast(list[Token], tree.children[1:-1])
        if arguments == [None]:
            arguments = []
        self.macros[name] = Macro(
            [str(i) for i in arguments], cast(Tree[Token], tree.children[-1])
        )

    def declr_block_macro(self, tree: Tree[Token]):
        name = cast(Token, tree.children[0])
        if name in self.block_macros:
            raise RangeError(name, "Redeclaration of macro", "Rename this macro")
        arguments = cast(list[Token], tree.children[1:-1])
        if arguments == [None]:
            arguments = []
        body = cast(Tree[Token], tree.children[-1])

        self.block_macros[name] = BlockMacro([str(i) for i in arguments], body)

    def declr_function_nowarp(self, tree: Tree[Token]):
        return self.declr_function(tree, warp=False)

    def declr_comment(self, tree: Tree[Token]):
        comment: str = literal(cast(Token, tree.children[0]))
        self.sprite.comment = comment
