from __future__ import annotations
from typing import TypeVar, Callable
from pathlib import Path
import lark.exceptions
from lark.lexer import Token
from . import term as t


class Error(Exception):
    def __init__(self, description: str, help: str | None = None):
        self.description = description
        self.help = help

    def print(self):
        t.w(t.brred)
        t.w("ERROR: ")
        t.w(self.description)
        if self.help:
            t.w("\n")
            t.w(t.brpink)
            t.w("help: ")
            t.w(self.help)
        t.w("\n")


class FileError(Error):
    def __init__(
        self, description: str, help: str | None = None, file: Path | None = None
    ):
        self.description = description
        self.help = help
        self.file = file

    def print(self):
        assert isinstance(self.file, Path)
        t.w(t.brred)
        t.w("ERROR: ")
        t.w(self.description)
        t.w("\n")
        t.w(t.brgreen)
        t.w(f"{self.file}")
        if self.help:
            t.w("\n")
            t.w(t.brpink)
            t.w("help: ")
            t.w(self.help)
        t.w("\n")


class TokenError(Error):
    def __init__(
        self,
        description: str,
        token: Token,
        help: str | None = None,
        file: Path | None = None,
    ):
        super().__init__(description, help)
        self.token = token
        self.file = file

    def print(self):
        assert isinstance(self.file, Path)
        assert isinstance(self.token.line, int)
        assert isinstance(self.token.column, int)
        with self.file.open("r") as file:
            iter = enumerate(file)
            while (line := next(iter))[0] < self.token.line - 1:
                continue
            line = line[1]
        t.w(f"Error! {t.brred}{self.description}{t.reset}\n")
        t.w(f"in {t.brblue}{self.file}:{self.token.line}:{self.token.column}\n")
        t.w(f"{t.brblack}{self.token.line: 4} | {t.reset}{line}")
        t.w(f"{t.brpink}{' '*(6+self.token.column)}{'^'*len(self.token)}")
        t.w(((" " + self.help) if self.help else "") + "\n")
        t.w(t.reset)


T = TypeVar("T")


def wrap_lark_errors(func: Callable[[], T], file: Path) -> T:
    try:
        return func()
    except lark.exceptions.VisitError as e:
        if isinstance(e.orig_exc, TokenError | FileError):
            e.orig_exc.file = file
            raise e.orig_exc from e
        raise e.orig_exc from e
    except TokenError as e:
        e.file = file
        raise e from e
    except FileError as e:
        e.file = file
        raise e from e
    except lark.exceptions.UnexpectedToken as e:
        raise TokenError(
            "Unexpected token",
            e.token,
            "Expected one of: " + ", ".join(e.expected),
            file,
        ) from e
    except lark.exceptions.UnexpectedCharacters as e:
        token = Token("NULL", "#", None, e.line, e.column)
        help = ""
        if "BANG" in e.allowed:
            help = ", Macro syntax has changed, use the syntax macro foo!() -> bar;"
        raise TokenError(
            "Unexpected characters",
            token,
            "Expected one of: " + ", ".join(e.allowed) + help,
            file,
        ) from e
