import importlib.resources

from lark.lark import Lark
from lark.lexer import Token

gparser = Lark(importlib.resources.open_text("res", "grammar.lark"))


def literal(literal: Token) -> str:
    if literal.type == "STRING":
        return literal[1:-1].replace(r"\\", "\\").replace(r"\"", '"')
    if literal.type == "NUMBER":
        return str(int(literal))
    if literal.type == "FLOAT":
        return str(float(literal))
    if literal.type == "ARGUMENT":
        return str(literal)[1:]
    if literal.type == "LCOMMENT":
        return str(literal)[2:-2]
    if literal.type == "CONST":
        return {"true": "1", "false": "0"}[str(literal)]
    raise ValueError(literal.type, literal)
