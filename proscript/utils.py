from pathlib import Path

from lark.lexer import Token


class gTokenException(Exception):
    def __init__(self, msg: str, token: Token):
        self.msg = msg
        self.token = token

    def print(self, file: Path):
        print("Error: " + self.msg)
        print(f"{file}:{self.token.line}:{self.token.column}")
        with file.open("r") as fp:
            for i, line in enumerate(fp.readlines()):
                if i == self.token.line - 1:
                    print(line, end="")
                    print(" " * (self.token.column - 1) + "^" * len(self.token))
                    break
