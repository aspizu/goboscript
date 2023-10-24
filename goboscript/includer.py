from __future__ import annotations
from typing import TYPE_CHECKING
from lark.tree import Tree
from lark.lexer import Token
from lark.visitors import Transformer
from .lib import EXT
from .parser import parser, literal

if TYPE_CHECKING:
    from pathlib import Path


class Includer(Transformer[Token, Tree[Token]]):
    def __init__(self, project: Path):
        super().__init__()
        self.project = project

    def declr_use(self, args: tuple[Token]) -> Tree[Token]:
        path = self.project / f"{literal(args[0])}.{EXT}"
        return parser.parse(path.read_text())
