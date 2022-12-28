import importlib.resources

from lark import Lark, Token

gparser = Lark(importlib.resources.open_text("res", "grammar.lark"), parser="lalr")


def literal(literal: Token) -> str:
    if literal.type == "STRING":
        return literal[1:-1].replace(r"\\", "\\").replace(r"\"", '"')
    raise ValueError(literal.type, literal)
