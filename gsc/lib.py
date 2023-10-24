from __future__ import annotations
import sys
import itertools
from typing import TYPE_CHECKING
from typing import Mapping
from typing import TypeVar
from typing import Iterable
from typing import Sequence
from difflib import get_close_matches
from lark import Token

if TYPE_CHECKING:
    from pathlib import Path

EXT = "gobo"
JSON = Mapping[str, "JSON"] | Sequence["JSON"] | str | int | float | bool | None

T = TypeVar("T")


def tripletwise(iterable: Iterable[T]) -> Iterable[tuple[T | None, T, T | None]]:
    a, b, c = itertools.tee(iterable, 3)
    next(c, None)
    return zip(itertools.chain([None], a), b, itertools.chain(c, [None]))


def file_suggest(path: Path) -> list[Path]:
    return [
        path.parent / match
        for match in get_close_matches(
            path.name,
            [subpath.name for subpath in path.parent.iterdir() if subpath.is_file()],
        )
    ]


def dir_suggest(path: Path) -> list[Path]:
    return [
        path.parent / match
        for match in get_close_matches(
            path.name,
            [subpath.name for subpath in path.parent.iterdir() if subpath.is_dir()],
        )
    ]


def num_plural(num: int, word: str) -> str:
    return (str(num) if num > 0 else "no") + word + ("s" if num > 1 else "")


def number(number: str) -> int | float:
    if number[:2] == "0x":
        return int(number[2:], 16)
    try:
        return int(number)
    except ValueError:
        return float(number)


def tok(string: str) -> Token:
    return Token("tok", value=string)


class Watcher:
    def __init__(self, files: list[Path]):
        self._files = files
        self._mtimes = [0.0] * len(self._files)

    def watch(self):
        try:
            self._watch()
        except KeyboardInterrupt:
            print("Bye...")
            sys.exit(0)

    def _watch(self):
        while True:
            for i, file in enumerate(self._files):
                mtime = file.stat().st_mtime
                if mtime != self._mtimes[i]:
                    self.on_change(file)
                self._mtimes[i] = mtime

    def on_change(self, file: Path) -> None:
        raise NotImplementedError
