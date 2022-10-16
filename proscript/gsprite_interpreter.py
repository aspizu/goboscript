from parser import strtoken
from pathlib import Path

from lark.visitors import Interpreter

from gblock_transformer import gBlockTransformer
from scratch import *
from utils import gTokenException


class gSpriteInterpreter(Interpreter):
    def __init__(self, loc: Path):
        self.loc = loc
        self.costumes: list[str] = []
        self.sounds: list[str] = []
        self.blocks: list[gBlock] = []
        self.variables: list[str] = []
        self.lists: list[str] = []
        self.procs: dict[str, gProcDef] = {}
        super().__init__()

    def to_gsprite(self, name: str) -> gSprite:
        return gSprite(
            name,
            tuple(gVariable(i, "gVariable") for i in self.variables),
            tuple(gList(i, ()) for i in self.lists),
            tuple(self.blocks),
            tuple(gCostume.from_path(self.loc / i) for i in self.costumes),
            tuple(gSound.from_path(self.loc / i) for i in self.sounds),
        )

    def declr_hat(self, node):
        self.blocks.append(gBlockTransformer(self.procs).transform(node))

    def declr_costumes(self, node):
        for i in node.children:
            if not (self.loc / strtoken(i)).is_file():
                raise gTokenException("file not found", i)
            self.costumes.append(strtoken(i))

    def declr_proc(self, node):
        warp: bool = node.children[0]
        name: str = node.children[1]
        args: tuple[str, ...] = tuple(node.children[2:-1]) or ()
        proc = gProcDef(
            name, args, warp, gBlockTransformer(self.procs).transform(node.children[-1])
        )
        self.blocks.append(proc)
        self.procs[name] = proc
