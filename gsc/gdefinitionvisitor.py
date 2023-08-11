from pathlib import Path
from typing import NamedTuple, cast

from gerror import gTokenError
from gparser import literal
from lark.lexer import Token
from lark.tree import Tree
from lark.visitors import Interpreter, Visitor
from lib import file_suggest, tok
from sb3 import gCostume, gList, gSprite, gVariable


class gFunction(NamedTuple):
    name: Token
    warp: bool
    arguments: list[Token]
    locals: list[str]
    proccode: str | None = None


class gMacro(NamedTuple):
    arguments: list[str]
    body: Tree[Token]


class BlockMacro(NamedTuple):
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
        token = cast(Token, tree.children[0])
        self.locals.append(str(token))
        qualname = f"{self.name}.{token}"
        self.sprite.variables[qualname] = gVariable(token)

    def varset(self, tree: Tree[Token]):
        token = cast(Token, tree.children[0])
        qualname = str(token)

        if qualname in self.locals:
            return

        if qualname in self.sprite.variables:
            return

        if qualname in self.sprite.lists:
            return

        self.sprite.variables[qualname] = gVariable(token)

    def listset(self, tree: Tree[Token]):
        token = cast(Token, tree.children[0])
        qualname = str(token)

        if qualname in self.sprite.lists:
            return

        if qualname in self.sprite.variables:
            return

        self.sprite.lists[qualname] = gList(token)


class gDefinitionVisitor(Interpreter[Token, None]):
    def __init__(self, project: Path, sprite: gSprite, tree: Tree[Token]):
        super().__init__()
        self.project = project
        self.sprite = sprite
        self.macros: dict[Token, gMacro] = {}
        self.block_macros: dict[Token, BlockMacro] = {}
        self.functions: dict[str, gFunction] = {
            # Scratch Addons Debugger blocks
            "breakpoint": gFunction(
                tok("breakpoint"),
                False,
                [],
                [],
                proccode="\u200B\u200Bbreakpoint\u200B\u200B",
            ),
            "log": gFunction(
                tok("log"),
                False,
                [tok("message")],
                [],
                proccode="\u200B\u200Blog\u200B\u200B %s",
            ),
            "warn": gFunction(
                tok("warn"),
                False,
                [tok("message")],
                [],
                proccode="\u200B\u200Bwarn\u200B\u200B %s",
            ),
            "error": gFunction(
                tok("error"),
                False,
                [tok("message")],
                [],
                proccode="\u200B\u200Berror\u200B\u200B %s",
            ),
        }
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
        token = cast(Token, tree.children[0])
        qualname = str(token)
        path = cast(Token, tree.children[1])
        file: Path = self.project / literal(path)
        if not file.is_file():
            matches = file_suggest(file)
            raise gTokenError(
                "Data file not found.",
                path,
                f"Did you mean {matches[0].relative_to(self.project)}?"
                if matches
                else None,
            )

        self.sprite.lists[qualname] = gList(token, file.read_text().splitlines())

    def imagelist(self, tree: Tree[Token]) -> None:
        from PIL import Image

        token = cast(Token, tree.children[0])
        qualname = str(token)
        path = cast(Token, tree.children[1])
        format = cast(Token | None, tree.children[2])
        file: Path = self.project / literal(path)
        if not file.is_file():
            matches = file_suggest(file)
            raise gTokenError(
                "Data file not found.",
                path,
                f"Did you mean {matches[0].relative_to(self.project)}?"
                if matches
                else None,
            )

        image = Image.open(file)
        if format is None:
            data = list(image.tobytes())  # type: ignore
        else:
            raise gTokenError(
                "Invalid imagelist format.",
                format,
                "Formats are not implemented yet, open a issue at https://github/aspizu/goboscript/issues",
            )
        self.sprite.lists[qualname] = gList(token, list(map(str, data)))

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

    def declr_block_macro(self, tree: Tree[Token]):
        name = cast(Token, tree.children[0])
        if name in self.block_macros:
            raise gTokenError("Redeclaration of macro", name, "Rename this macro")
        arguments = cast(list[Token], tree.children[1:-1])
        body = cast(Tree[Token], tree.children[-1])

        self.block_macros[name] = BlockMacro([str(i) for i in arguments], body)

    def declr_function_nowarp(self, tree: Tree[Token]):
        return self.declr_function(tree, False)

    def declr_comment(self, tree: Tree[Token]):
        comment: str = literal(cast(Token, tree.children[0]))
        self.sprite.comment = comment
