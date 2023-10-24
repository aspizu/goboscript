from __future__ import annotations
from typing import TYPE_CHECKING
from typing import cast
from importlib.resources import files
import res
from sb3 import Sprite
from gerror import FileError
from gparser import gparser
from gincluder import gIncluder
from lark.lexer import Token
from sb3.cleanup import cleanup
from lark.visitors import Interpreter
from gblocktransformer import BlockTransformer
from gmacrotransformer import MacroTransformer
from gmacrotransformer import BlockMacroVisitor
from gdefinitionvisitor import DefinitionVisitor

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
        tree.children.insert(
            0, gparser.parse((files(res) / "standard_library.gs").read_text())
        )
        tree = gIncluder(project).transform(tree)
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
    declr_onclone = declr_on
    declr_ontimer = declr_on
    declr_onloudness = declr_on
    declr_onkey = declr_on
    declr_onbackdrop = declr_on
