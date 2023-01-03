import itertools
from difflib import get_close_matches
from pathlib import Path
from typing import Iterable, TypeVar

EXT = "gs"
JSON = dict[str, "JSON"] | list["JSON"] | str | int | float | bool | None

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
    try:
        return int(number)
    except:
        return float(number)
