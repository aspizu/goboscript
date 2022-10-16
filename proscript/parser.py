from importlib.resources import open_text

from lark import Lark  # type: ignore
from lark.lexer import Token

parser = Lark(open_text("resources", "grammar.lark"))


def strtoken(token: Token) -> str:
    if token.type == "STRING":
        return str(token)[1:-1]
    else:
        raise ValueError
