from __future__ import annotations
from typing import TYPE_CHECKING
from lark.tree import Tree
from lark.lexer import Token
from lark.visitors import Transformer
from .lib import EXT
from .parser import literal

if TYPE_CHECKING:
    from pathlib import Path
    from lark import Lark


class Includer(Transformer[Token, Tree[Token]]):
    def __init__(self, project: Path, parser: Lark):
        super().__init__()
        self.parser = parser
        self.project = project

    def declr_use(self, args: tuple[Token]) -> Tree[Token]:
        path = self.project / f"{literal(args[0])}.{EXT}"
        return self.parser.parse(path.read_text())
