from __future__ import annotations
from typing import TYPE_CHECKING
from dataclasses import dataclass
from lark import Lark, Token

if TYPE_CHECKING:
    from pathlib import Path


def literal(literal: Token) -> str:
    if literal.type == "STRING":
        return literal[1:-1].replace(r"\\", "\\").replace(r"\"", '"')
    raise ValueError(literal.type, literal)


preproc_grammar = r"""
?start: "%" "use" STRING -> import_stmt
NAME: /[_a-zA-Z][_a-zA-Z0-9]*/
STRING: /"([^"\\]|\\.)*"/
NUMBER: /-?[0-9]+/
FLOAT: /-?[0-9]+\.[0-9]+/
COMMENT: /\/\*(\*(?!\/)|[^*])*\*\//
SCOMMENT: /\/\/.*/
%ignore " "
%ignore "\n"
%ignore "\t"
%ignore COMMENT
%ignore SCOMMENT
"""

preproc = Lark(preproc_grammar)


@dataclass
class Range:
    pasted_start: int
    file_start: int
    length: int
    file: Path

    def get_file_line(self, line: int):
        return self.file_start + line - self.pasted_start


@dataclass
class Paste:
    lines: list[str]
    ranges: list[Range]

    def get_range_from_line(self, line: int):
        for range in self.ranges:
            if range.pasted_start <= line < range.pasted_start + range.length:
                return range
        return None


class PasteBuilder:
    def __init__(self, relative: Path):
        self.relative = relative
        self.paste = Paste([], [])
        self.start = 0
        self.i = 0

    def include(self, file: Path):
        file_start = 0
        for file_i, line in enumerate(file.open()):
            if line.startswith("%"):
                self.paste.ranges.append(
                    Range(self.start, file_start, self.i - self.start, file)
                )
                self.start = self.i
                tree = preproc.parse(line)
                match tree.data:
                    case "import_stmt":
                        token = tree.children[0]
                        assert isinstance(token, Token)
                        path = literal(token)
                        if "*" in path:
                            for ifile in self.relative.glob(path):
                                self.include(ifile)
                        else:
                            ifile = self.relative / path
                            self.include(ifile)
                        file_start = file_i + 1
                    case _:
                        raise ValueError(tree)
            else:
                self.paste.lines.append(line)
                self.i += 1
        self.paste.ranges.append(
            Range(self.start, file_start, self.i - self.start, file)
        )
        self.start = self.i
        return self


def print_paste(paste: Paste):
    from rich import print

    for r in paste.ranges:
        for i in range(r.pasted_start, r.pasted_start + r.length):
            print(
                f"{1+i: 2} | {1+r.file_start+i-r.pasted_start: 2} | {paste.lines[i]}",
                end="",
            )
