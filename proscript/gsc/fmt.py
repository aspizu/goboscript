from pathlib import Path

import lark
from rich import print as rprint

from common import parser


def format_file(file_pth: Path) -> None:
    tree = parser.parse(file_pth.read_text())
    print(" -- TREE -- ")
    rprint(tree)
    formatted = Formatter().visit(tree)
    print(" -- FORMATTED -- ")
    rprint(formatted)


class Formatter(lark.visitors.Interpreter):

    def visit(self, tree):
        if isinstance(tree, lark.Tree):
            return super().visit(tree)
        else:
            tree: lark.Token
            return getattr(self, tree.type)(tree)

    def start(self, tree):
        return "\n".join(map(self.visit, tree.children))

    def costumes(self, tree):
        return f"costumes {', '.join(map(self.visit, tree.children))};"

    def hat(self, tree):
        return f"{tree.children[0]} {tree.children[1]}"

    def stack(self, tree):
        return f"{{{';'.join(map(self.visit, tree.children))}}}"

    def statement(self, tree):
        return f"{tree.children[0]} {', '.join(map(self.visit, tree.children[1:]))};"
