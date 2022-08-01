import importlib.resources
from pathlib import Path
from typing import Optional

import gobomatic as gm
import lark

import tcol

STDLIB_PTH = Path(__file__).parent / "stdlib"
parser = lark.Lark(importlib.resources.read_text("resources", "grammar.lark"))


def gm_bool(tree):
    if isinstance(tree, gm.BooleanReporterBlock):
        return tree
    else:
        return gm.Not(gm.Eq(tree, 0))


def Token_STRING_to_str(string: lark.Token) -> str:
    return str(string)[1:-1]


class CompilerExceptions(Exception):
    def print(self):
        ...


class CompilerError(CompilerExceptions):
    def __init__(self, msg: str, tip: str = None):
        self.msg = msg
        self.tip = tip
        super().__init__()

    def print(self):
        print(
            f"{tcol.red}Error{tcol.off} : {self.msg}\n"
            f"{tcol.yellow}{self.tip}{tcol.off or ''}\n"
        )


class CompilerFileError(CompilerExceptions):
    def __init__(self, token: lark.Token, msg: str, tip: str = None):
        self.token = token
        self.msg = msg
        self.tip = tip
        self.file_pth: Optional[Path] = None
        super().__init__()

    def print(self):
        print(
            f"{tcol.brred}Error{tcol.off} : {self.msg}\n"
            f"at {tcol.green}{self.file_pth.as_posix()}"
            f":{self.token.line}:{self.token.column}{tcol.off}\n"
            f"{self.token.line:>5} | "
            f"{self.file_pth.read_text().splitlines()[self.token.line-1]}\n"
            f"        {' '*(self.token.column-1)}"
            f"{tcol.brpink}{'^'*len(self.token)}{tcol.off}"
            f"{tcol.yellow}{self.tip or ''}{tcol.off}\n"
        )
