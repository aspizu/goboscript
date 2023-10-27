from __future__ import annotations
from typing import TYPE_CHECKING, TypeVar, Callable
from pathlib import Path
from dataclasses import dataclass
import lark.exceptions
from lark.lexer import Token
from . import term as t

if TYPE_CHECKING:
    from .paste import Paste


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


@dataclass
class Range:
    line: int
    column: int
    length: int


class RangeError(Error):
    def __init__(
        self,
        token: Token,
        description: str,
        help: str | None = None,
        file: Path | None = None,
    ):
        super().__init__(description, help)
        self.range = Range(token.line, token.column, len(token))
        self.file = file

    def print(self):
        assert isinstance(self.file, Path)
        with self.file.open("r") as file:
            iter = enumerate(file)
            while (line := next(iter))[0] < self.range.line - 1:
                continue
            line = line[1]
        t.w(f"Error! {t.brred}{self.description}{t.reset}\n")
        t.w(f"in {t.brblue}{self.file}:{self.range.line}:{self.range.column}\n")
        t.w(f"{t.brblack}{self.range.line: 4} | {t.reset}{line}")
        t.w(f"{t.brpink}{' '*(6+self.range.column)}{'^'*self.range.length}")
        t.w(((" " + self.help) if self.help else "") + "\n")
        t.w(t.reset)


T = TypeVar("T")


def wrap_lark_errors(func: Callable[[], T], paste: Paste, file: Path) -> T:
    try:
        return func()
    except lark.exceptions.VisitError as e:
        match e.orig_exc:
            case RangeError():
                line = e.orig_exc.range.line
                assert line is not None
                range = paste.get_range_from_line(line - 1)
                assert range is not None
                e.orig_exc.file = range.file
                e.orig_exc.range.line = range.get_file_line(line)
            case FileError():
                e.orig_exc.file = file
            case _:
                pass
        raise e.orig_exc from e
    except RangeError as e:
        e.file = file
        raise e from e
    except FileError as e:
        e.file = file
        raise e from e
    except lark.exceptions.UnexpectedToken as e:
        raise RangeError(
            e.token,
            "Unexpected token",
            "Expected one of: " + ", ".join(e.expected),
            file,
        ) from e
    except lark.exceptions.UnexpectedCharacters as e:
        token = Token("NULL", "#", None, e.line, e.column)
        if e.char == ";":
            raise RangeError(
                token,
                "Use of semicolons",
                "Use the command-line argument --semi to enable old syntax.",
                file,
            ) from e
        raise RangeError(
            token,
            "Unexpected characters",
            "Expected one of: " + ", ".join(e.allowed),
            file,
        ) from e
