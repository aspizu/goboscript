from __future__ import annotations
import itertools
from typing import TYPE_CHECKING, TypeVar, Callable
from pathlib import Path
from dataclasses import dataclass
import lark.exceptions
from . import term as t

if TYPE_CHECKING:
    from importlib.abc import Traversable
    from lark.lexer import Token
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

    @classmethod
    def new(cls, token: Token):
        return cls(token.line - 1, token.column - 1, len(token))


class RangeError(Error):
    def __init__(
        self,
        range: Token | Range,
        description: str,
        help: str | None = None,
        file: Path | Traversable | None = None,
    ):
        super().__init__(description, help)
        self.range = range if isinstance(range, Range) else Range.new(range)
        self.file = file
        self.includepath: Path | Traversable | None = None
        self.includerange: Range | None = None

    def print(self):
        assert isinstance(self.file, Path)
        t.w(f"Error! {t.brred}{self.description}{t.reset}\n")
        if self.includepath and self.includerange:
            t.w(
                f"in file included from {t.brblue}"
                f"{self.includepath}:{self.includerange.line+1}:{self.includerange.column+1}\n"
            )
            line = next(
                itertools.islice(
                    iter(self.includepath.open()), self.includerange.line, None
                )
            )
            t.w(f"{t.brblack}{self.includerange.line+1: 4} | {t.reset}{line}")
        t.w(f"in {t.brblue}{self.file}:{self.range.line+1}:{self.range.column+1}\n")
        line = next(itertools.islice(iter(self.file.open()), self.range.line, None))
        t.w(f"{t.brblack}{self.range.line+1: 4} | {t.reset}{line}")
        t.w(f"{t.brpink}{' '*(7+self.range.column)}{'^'*self.range.length}")
        t.w(((" " + self.help) if self.help else "") + "\n")
        t.w(t.reset)


T = TypeVar("T")


def wrap_lark_errors(func: Callable[[], T], file: Path | Traversable) -> T:
    try:
        return func()
    except lark.exceptions.UnexpectedToken as e:
        raise RangeError(
            e.token,
            "Unexpected token",
            "Expected one of " + ", ".join(e.expected),
            file,
        ) from e
    except lark.exceptions.UnexpectedCharacters as e:
        raise RangeError(
            Range(e.line - 1, e.column - 1, 1),
            "Unexpected characters",
            "Expected one of " + ", ".join(e.allowed),
            file,
        ) from e
    except lark.exceptions.UnexpectedEOF as e:
        raise RangeError(
            Range(e.line - 1, e.column - 1, 1),
            "Unexpected end of file",
            "Expected one of " + ", ".join(e.expected),
            file,
        ) from e


def wrap_and_resolve_errors_in_paste(
    func: Callable[[], T], paste: Paste, file: Path
) -> T:
    try:
        return func()
    except lark.exceptions.VisitError as e:
        raise resolve_errors_in_paste(e.orig_exc, paste, file) from e
    except RangeError as e:
        resolve_errors_in_paste(e, paste, file)
        raise
    except FileError as e:
        resolve_errors_in_paste(e, paste, file)
        raise


def resolve_errors_in_paste(exception: Exception, paste: Paste, file: Path):
    match exception:
        case RangeError():
            range = paste.get_range_from_line(exception.range.line)
            assert range is not None
            exception.file = range.file
            exception.includepath = range.includepath
            exception.includerange = range.includerange
            exception.range.line = range.get_file_line(exception.range.line)
        case FileError():
            exception.file = file
        case _:
            pass
    return exception
