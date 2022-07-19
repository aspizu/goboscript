import lark
from lark import Tree, Token
from blocks import STATEMENT_BLOCKS, REPORTER_BLOCKS
import interpreter
import gobomatic as gm


def Token_STRING_to_str(string) -> str:
    return str(string)[1:-1]


class BlockTransformer(lark.Transformer):
    def __init__(self, state: "interpreter.Interpreter"):
        self.state = state

    def STRING(self, token: Token):
        return Token_STRING_to_str(token)

    def NUMBER(self, token: Token):
        return str(token)

    def BOOL(self, token: Token):
        return "1" if token == "true" else "0"

    def stack(self, args):
        return args

    def statement(self, args):
        name: Token = args[0]
        args: list[Tree] = args[1:]
        if args[0] is None:
            del args[0]
        try:
            return STATEMENT_BLOCKS[name](*args)
        except KeyError:
            return self.state.funcs[name](*args)

    def reporter(self, args):
        name: Token = args[0]
        args: list[Tree] = args[1:]
        if args[0] is None:
            del args[0]
        return REPORTER_BLOCKS[name](*args)

    def argument(self, args):
        name: Token = args[0]
        return getattr(gm.Arg, name)

    def varset(self, args):
        name: Token = args[0]
        expr: Tree = args[1]
        if name not in self.state.vars:
            self.state.vars[name] = self.state.sprite.Var(name)
        return self.state.vars[name].set(expr)

    def var(self, args):
        name: Token = args[0]
        if name in self.state.vars:
            return self.state.vars[name]

    def iff(self, args):
        condition: Tree = args[0]
        stack: Tree = args[1]
        return gm.If(condition)(*stack)

    def ifelse(self, args):
        condition: Tree = args[0]
        stack1: Tree = args[1]
        stack2: Tree = args[2]
        return gm.If(condition)(*stack1).Else(*stack2)

    def repeat(self, args):
        expr: Tree = args[0]
        stack: Tree = args[1]
        return gm.Repeat(expr)(*stack)

    def until(self, args):
        condition: Tree = args[0]
        stack: Tree = args[1]
        return gm.Until(condition)(*stack)

    def ofstatement(self, args):
        name1: Token = args[0]
        name2: Token = args[1]
        args: list[Tree] = args[2:]
        if args[0] is None:
            del args[0]
        if name1 in self.state.vars:
            return getattr(self.state.vars[name1], name2)(*args)
        else:
            return getattr(self.state.lsts[name1], name2)(*args)

    ofreporter = ofstatement

    def lstset(self, args):
        name: Token = args[0]
        if name not in self.state.lsts:
            self.state.lsts[name] = self.state.sprite.List(name)
        return self.state.lsts[name].delete_all()
