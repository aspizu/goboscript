from pathlib import Path

from lark.lexer import Token

__all__ = ("gError", "gCodeError")


class gError(Exception):
    def __init__(self, msg: str):
        self.msg = msg

    def print(self):
        print(self.msg)


class gCodeError(Exception):
    def __init__(self, token: Token, msg: str):
        self.token = token
        self.msg = msg

    def print(self, file: Path):
        with file.open("r") as fp:
            file_text = fp.readlines()
        print(file_text[self.token.line - 1], end="")
        print((self.token.column - 1) * " " + "^" * len(self.token))
        print(self.msg)
