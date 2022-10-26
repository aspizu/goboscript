import importlib.resources

from lark.lark import Lark
from lark.lexer import Token

gparser = Lark(importlib.resources.open_text("gparser", "grammar.lark"))


def parse_token(token: Token) -> str:
    if token.type == "STRING":
        return str(token)[1:-1]
    raise TypeError(token, token.type)
