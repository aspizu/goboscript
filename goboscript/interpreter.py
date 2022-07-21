import typing
from typing import Any
import lark
import gobomatic as gm
from lark.tree import Tree
from lark.lexer import Token
from pathlib import Path
from blocktransformer import BlockTransformer, Token_STRING_to_str


class Interpreter(lark.visitors.Interpreter):
    def __init__(self, sprite: gm.Sprite, sprite_pth: Path):
        self.sprite = sprite
        self.sprite_pth = sprite_pth
        self.vars: dict[Token, gm.Var] = {}
        self.lsts: dict[Token, gm.List] = {}
        self.funcs: dict[Token, gm.Sprite.FuncFactory] = {}

    def costumes(self, tree: Tree):
        costumes: list[str] = [
            self.sprite_pth.parent.as_posix() + "/" + Token_STRING_to_str(i)
            for i in tree.children
        ]
        self.sprite.costumes = costumes

    def sounds(self, tree: Tree):
        sounds: list[str] = [
            self.sprite_pth.parent.as_posix()
            + "/"
            + Token_STRING_to_str(typing.cast(Token, i))
            for i in tree.children
        ]
        self.sprite.sounds = sounds

    def hat(self, tree: Tree):
        name = typing.cast(Token, tree.children[0])
        args = tree.children[1:-1]
        if args[0] is None:
            del args[0]
        stack = BlockTransformer(self).transform(tree.children[-1])
        if name == "onflag":
            self.sprite.WhenFlagClicked(*stack)
        elif name == "onkey":
            self.sprite.WhenKeyPressed(Token_STRING_to_str(args[0]))(*stack)
        elif name == "onevent":
            self.sprite.WhenReceived(Token_STRING_to_str(args[0]))(*stack)
        elif name == "onclone":
            self.sprite.WhenStartAsClone(*stack)
        elif name == "onclick":
            self.sprite.WhenThisSpriteClicked(*stack)
        elif name == "ontimer":
            self.sprite.WhenTimerGreaterThan(
                BlockTransformer(self).transform(typing.cast(Tree[Any], args[0]))
            )(*stack)
        elif name == "onbackdrop":
            self.sprite.WhenBackdropSwitchesTo(Token_STRING_to_str(args[0]))(*stack)
        elif name == "onloudness":
            self.sprite.WhenLoudnessGreaterThan(Token_STRING_to_str(args[0]))(*stack)

    def deff(self, tree: Tree):
        warp: bool = tree.children[0] != "nowarp"
        name = typing.cast(Token, tree.children[1])
        args = typing.cast(list[Token], tree.children[2:-1])
        stack = tree.children[-1]
        if args[0] is None:
            del args[0]
        if name not in self.funcs:
            self.funcs[name] = self.sprite.Func(
                *[getattr(gm.Arg, i) for i in args], name=name, warp=warp
            )
            self.funcs[name].Define(*BlockTransformer(self, allowed_args=args).transform(stack))
