from pathlib import Path
from typing import Callable, TypeVar, cast

import lark.exceptions
import term as t
from lark import Token


class gError(Exception):
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


class gFileError(gError):
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


class gTokenError(gError):
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
        line = self.file.read_text().split("\n")[cast(int, self.token.line) - 1]
        t.w(t.brred)
        t.w("ERROR: ")
        t.w(self.description)
        t.w("\n")
        t.w(t.brgreen)
        t.w(f"{self.file}:{self.token.line}:{self.token.column}")
        t.w("\n")
        t.w(t.brblack)
        t.w(str(self.token.line).rjust(8))
        t.w(" | ")
        t.w(t.reset)
        t.w(line)
        t.w("\n")
        t.w(t.brpink)
        t.w(" " * (cast(int, self.token.column) + 10))
        t.w("^" * len(self.token))
        if self.help:
            t.w(" help: ")
            t.w(self.help)
        t.w("\n")


T = TypeVar("T")


def wrap_lark_errors(func: Callable[[], T], file: Path) -> T:
    try:
        return func()
    except lark.exceptions.VisitError as e:
        if isinstance(e.orig_exc, gTokenError | gFileError):
            e.orig_exc.file = file
            raise e.orig_exc
        if isinstance(e.orig_exc, gError):
            raise e.orig_exc
        raise e
    except gTokenError as e:
        e.file = file
        raise e
    except lark.exceptions.UnexpectedToken as e:
        raise gTokenError("Unexpected token", e.token, file=file)
