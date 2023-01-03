from difflib import get_close_matches
from typing import Any

from gdefinitionvisitor import gMacro
from gerror import gTokenError
from lark import Token, Transformer, Tree


class gMacroTransformer(Transformer[Token, Tree[Token]]):
    def __init__(self, macros: dict[str, gMacro]):
        super().__init__()
        self.macros = macros

    def macro(self, args: list[Any]):
        name: Token = args[0]
        if name[1:] not in self.macros:
            matches = get_close_matches(name, self.macros.keys())
            raise gTokenError(
                f"Undefined macro `{name[1:]}`",
                name,
                f"Did you mean `{matches[0]}`?" if matches else None,
            )
        arguments: list[Tree[Token] | Token] = args[1:]
        return gMacroEvaluate(self.macros[name[1:]], arguments).get()


class gMacroEvaluate(Transformer[Token, Tree[Token]]):
    def __init__(self, macro: gMacro, arguments: list[Tree[Token] | Token]):
        super().__init__()
        self.macro = macro
        self.arguments = arguments

    def get(self):
        return self.transform(self.macro.body)

    def macrovar(self, args: tuple[Token]):
        return self.arguments[self.macro.arguments.index(str(args[0][1:]))]
