from pathlib import Path

from lark.tree import Tree
from lark.visitors import Interpreter, Visitor
from sb3 import *

from .gblock_transformer import gBlockTransformer
from .gexception import *
from .gparser import parse_token


class DefCollector(Visitor):
    def __init__(self, prefix: Path) -> None:
        self.procs: dict[str, list[str]] = {}
        self.variables: dict[str, gVariable] = {}
        self.global_variables: dict[str, gVariable] = {}
        self.lists: dict[str, gList] = {}
        self.global_lists: dict[str, gList] = {}
        self.costumes: list[Path] = []
        self.prefix = prefix
        super().__init__()

    def declr_costumes(self, tree):
        for i in tree.children:
            if "*" in str(i):
                paths = list(self.prefix.glob(parse_token(i)))
                if len(paths) == 0:
                    raise gCodeError(i, "Glob is empty")
                self.costumes.extend(paths)
            else:
                path = self.prefix / parse_token(i)
                if not path.is_file():
                    raise gCodeError(i, "File not found")
                self.costumes.append(path)

    def declr_proc(self, tree):
        self.procs[str(tree.children[1])] = [str(i) for i in tree.children[2:-1]]

    def block_setvar(self, tree):
        name = str(tree.children[0])
        if name[0] == "$":
            self.global_variables[name] = gVariable(name)
        else:
            self.variables[name] = gVariable(name)

    def block_setlist(self, tree):
        name = str(tree.children[0])
        if name[0] == "$":
            self.global_lists[name] = gList(name)
        else:
            self.lists[name] = gList(name)


class gSpriteInterpreter(Interpreter):
    def __init__(self, tree: Tree, prefix: Path) -> None:
        super().__init__()
        self.collector = DefCollector(prefix)
        self.collector.visit(tree)
        self.gblocktrans = gBlockTransformer(self.collector)
        if len(self.collector.costumes) == 0:
            raise gError("No costumes defined")
        self.blocks: list[gHatBlock] = []
        self.interpret(tree)

    def declr_hat(self, tree: Tree):
        self.blocks.append(self.gblocktrans.transform(tree))

    def to_gSprite(self, name: str) -> gSprite:
        return gSprite(
            name,
            list(self.collector.variables.values()),
            list(self.collector.lists.values()),
            self.blocks,
            [gCostume(i) for i in self.collector.costumes],
        )
