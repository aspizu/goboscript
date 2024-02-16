from __future__ import annotations
from typing import TYPE_CHECKING, cast
from lark.lexer import Token
from lark.visitors import Interpreter
from .lib import tok
from .sb3 import Sprite
from .error import FileError
from .sb3.block import Variable
from .sb3.cleanup import cleanup
from .blocktransformer import BlockTransformer
from .macrotransformer import MacroTransformer, BlockMacroVisitor
from .definitionvisitor import DefinitionVisitor

if TYPE_CHECKING:
    from pathlib import Path
    from lark.tree import Tree


class SpriteInterpreter(Interpreter[Token, None]):
    def __init__(
        self,
        project: Path,
        name: str,
        tree: Tree[Token],
        globals: list[str],
        listglobals: list[str],
    ):
        super().__init__()
        self.sprite = Sprite(name, {}, {}, [], [])
        self.gdefinitionvisitor = DefinitionVisitor(
            project, self.sprite, tree, globals, listglobals
        )
        if len(self.sprite.costumes) == 0:
            raise FileError("No costumes defined", "Add a costumes statement")
        BlockMacroVisitor(tree, self.gdefinitionvisitor.block_macros)
        tree = MacroTransformer(self.gdefinitionvisitor.macros).transform(tree)
        self.visit(tree)
        if self.sprite.name == "Stage":
            self.sprite.variables["( ͡° ͜ʖ ͡°)"] = Variable("( ͡° ͜ʖ ͡°)", tok(""))
        cleanup(self.sprite.blocks)

    def declr_on(self, tree: Tree[Token]):
        self.sprite.blocks.append(
            BlockTransformer(self.gdefinitionvisitor).transform(tree)
        )

    def declr_function(self, tree: Tree[Token]):
        prototype = self.gdefinitionvisitor.functions[cast(Token, tree.children[0])]
        self.sprite.blocks.append(
            BlockTransformer(self.gdefinitionvisitor, prototype).transform(tree)
        )

    declr_function_nowarp = declr_function
    declr_onflag = declr_on
    declr_onclick = declr_on
    declr_onclone = declr_on
    declr_ontimer = declr_on
    declr_onloudness = declr_on
    declr_onkey = declr_on
    declr_onbackdrop = declr_on
