from lark.lexer import Token

__all__ = ("gError", "gCodeError")


class gError(Exception):
    def __init__(self, msg: str):
        self.msg = msg


class gCodeError(Exception):
    def __init__(self, token: Token, msg: str):
        self.token = token
        self.msg = msg
