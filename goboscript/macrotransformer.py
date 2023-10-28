from __future__ import annotations
from copy import deepcopy
from typing import TYPE_CHECKING, Any, cast
from difflib import get_close_matches
from lark import Visitor
from lark.tree import Tree
from lark.lexer import Token
from lark.visitors import Transformer
from .error import RangeError

if TYPE_CHECKING:
    from .definitionvisitor import Macro, BlockMacro


class BlockMacroVisitor(Visitor[Token]):
    def __init__(self, tree: Tree[Token], macros: dict[Token, BlockMacro]):
        self.macros = macros
        self.visit(tree)

    def stack(self, node: Tree[Token]):
        apply: list[int] = []
        for i, child in enumerate(node.children):
            if isinstance(child, Tree) and child.data == "block_macro":
                # Collect index of every block_macro usage
                apply.append(i)

        for i in apply:
            usage = cast(Tree[Token], node.children[i])
            name = cast(Token, usage.children[0])
            arguments = usage.children[1:]
            if arguments == [None]:
                arguments = []
            if name not in self.macros:
                matches = get_close_matches(name, self.macros.keys())
                raise RangeError(
                    name,
                    f"Undefined macro `{name}`",
                    f"Did you mean `{matches[0]}`?" if matches else None,
                )
            macro = self.macros[name]
            if len(arguments) < len(macro.arguments):
                raise RangeError(
                    name,
                    "Missing arguments for block macro",
                    help="Missing " + ", ".join(macro.arguments[len(arguments) :]),
                )
            if len(arguments) > len(macro.arguments):
                raise RangeError(
                    name,
                    "Too many arguments for block macro",
                    help="Expected " + ", ".join(macro.arguments),
                )
            body = deepcopy(macro.body)
            self.visit(body)
            stack = MacroEvaluate(macro, arguments).transform(body)
            node.children.pop(i)
            for child in reversed(stack.children):
                node.children.insert(i, child)


class MacroTransformer(Transformer[Token, Tree[Token]]):
    def __init__(self, macros: dict[Token, Macro]):
        super().__init__()
        self.macros = macros

    def macro(self, args: list[Tree[Token] | Token]):
        name = cast(Token, args[0])
        if name not in self.macros:
            matches = get_close_matches(name, self.macros.keys())
            raise RangeError(
                name,
                f"Undefined macro `{name}`",
                f"Did you mean `{matches[0]}`?" if matches else None,
            )
        macro = self.macros[name]
        arguments: list[Tree[Token] | Token] = args[1:]
        if arguments == [None]:
            arguments = []
        if len(arguments) < len(macro.arguments):
            raise RangeError(
                name,
                "Missing arguments for macro.",
                help="Missing " + ", ".join(macro.arguments[len(arguments) :]),
            )
        if len(arguments) > len(macro.arguments):
            raise RangeError(
                name,
                "Too many arguments for macro.",
                help="Expected " + ", ".join(macro.arguments),
            )
        return MacroEvaluate(macro, arguments).get(self)


class MacroEvaluate(Transformer[Token, Tree[Token]]):
    def __init__(self, macro: Macro | BlockMacro, arguments: list[Tree[Token] | Token]):
        super().__init__()
        self.macro__ = macro
        self.arguments = arguments

    def get(self, macros: MacroTransformer) -> Tree[Token] | Token:
        if isinstance(self.macro__.body, Token):
            return self.macro__.body
        return self.transform(macros.transform(self.macro__.body))

    def macrovar(self, args: Any):
        token: Token = args[0]
        return self.arguments[self.macro__.arguments.index(token[:-1])]
