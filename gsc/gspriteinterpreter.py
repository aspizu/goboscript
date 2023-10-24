from importlib.resources import files
from pathlib import Path
from typing import cast

import res
from gblocktransformer import gBlockTransformer
from gdefinitionvisitor import gDefinitionVisitor
from gerror import gFileError
from gincluder import gIncluder
from gmacrotransformer import BlockMacroVisitor, gMacroTransformer
from gparser import gparser
from lark.lexer import Token
from lark.tree import Tree
from lark.visitors import Interpreter
from sb3 import gSprite
from sb3.cleanup import cleanup


class gSpriteInterpreter(Interpreter[Token, None]):
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
        self.sprite = gSprite(name, {}, {}, [], [])
        self.gdefinitionvisitor = gDefinitionVisitor(
            project, self.sprite, tree, globals, listglobals
        )
        if len(self.sprite.costumes) == 0:
            raise gFileError("No costumes defined", "Add a costumes statement")
        BlockMacroVisitor(tree, self.gdefinitionvisitor.block_macros)
        tree = gMacroTransformer(self.gdefinitionvisitor.macros).transform(tree)
        self.visit(tree)
        cleanup(self.sprite.blocks)

    def declr_on(self, tree: Tree[Token]):
        self.sprite.blocks.append(
            gBlockTransformer(self.gdefinitionvisitor).transform(tree)
        )

    def declr_function(self, tree: Tree[Token]):
        prototype = self.gdefinitionvisitor.functions[cast(Token, tree.children[0])]
        self.sprite.blocks.append(
            gBlockTransformer(self.gdefinitionvisitor, prototype).transform(tree)
        )

    declr_function_nowarp = declr_function
    declr_onflag = declr_on
    declr_onclone = declr_on
    declr_ontimer = declr_on
    declr_onloudness = declr_on
    declr_onkey = declr_on
    declr_onbackdrop = declr_on
