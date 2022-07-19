from lark import Lark, Visitor, Transformer, Tree
from importlib import resources
import gobomatic as gm
from blocks import STATEMENT_BLOCKS, REPORTER_BLOCKS

parser = Lark(resources.read_text("data", "grammer.lark"), start="start")


parse = parser.parse


def tokentostr(token) -> str:
    return str(token)[1:-1]


class FirstPass(Visitor):
    def __init__(self, sprite, project_root):
        self.project_root = project_root
        self.sprite = sprite
        self.vars: dict[str, gm.Var] = {}
        self.lsts: dict[str, gm.List] = {}
        self.funcs: dict[str, gm.Sprite.FuncFactory] = {}

    def costumes(self, node: Tree):
        if self.sprite.costumes[0] == "":
            del self.sprite.costumes[0]
        for i in node.children:
            self.sprite.costumes.append(f"{self.project_root}/{tokentostr(i)}")

    def sounds(self, node: Tree):
        for i in node.children:
            self.sprite.sounds.append(f"{self.project_root}/{tokentostr(i)}")

    def varset(self, node: Tree):
        var = str(node.children[0])
        if var not in self.vars:
            self.vars[var] = self.sprite.Var(var)

    def listset(self, node: Tree):
        lst = str(node.children[0])
        if lst not in self.lsts:
            self.lsts[lst] = self.sprite.List(lst)

    def procdef(self, node: Tree):
        warp: bool = node.children[0] is None
        name: str = str(node.children[1])
        if node.children[2] is None:
            del node.children[2]
        args: list[str] = [str(i) for i in node.children[2:-1]]
        if name not in self.funcs:
            func = self.sprite.Func(
                *[getattr(gm.Arg, i) for i in args], name=name, warp=warp
            )
            self.funcs[name] = func
        else:
            raise Exception


class SecondPass(Transformer):
    def __init__(
        self,
        sprite: gm.Sprite,
        vars: dict[str, gm.Var],
        lsts: dict[str, gm.List],
        funcs: dict[str, gm.Sprite.FuncFactory],
    ):
        self.sprite: gm.Sprite = sprite
        self.vars: dict[str, gm.Var] = vars
        self.lsts: dict[str, gm.List] = lsts
        self.funcs: dict[str, gm.Sprite.FuncFactory] = funcs

    def start(self, args):
        return [i for i in args if isinstance(i, gm.primitives.HatBlock)]

    def argument(self, args):
        return getattr(gm.Arg, args[0])

    def procdef(self, args):
        self.funcs[args[1]].Define(*args[-1])

    def varset(self, args: list):
        return args[0].set(args[1])

    def listset(self, args: list):
        return args[0].delete_all()

    def stack(self, args: list):
        return args

    def hat(self, args: list):
        if args[0] == "whenflagclicked":
            return gm.blocks.events.WhenFlagClicked(args[2])

    def STRING(self, token):
        return tokentostr(token)

    def NUMBER(self, token):
        return str(token)

    def BOOL(self, token):
        return "1" if token == "true" else "0"

    def NAME(self, token):
        try:
            return self.vars[token]
        except KeyError:
            try:
                return self.lsts[token]
            except KeyError:
                return token

    def statement(self, args: list):
        if args[1] is None:
            del args[1]
        try:
            return STATEMENT_BLOCKS[args[0]](*args[1:])
        except KeyError:
            return self.funcs[args[0]](*args[1:])

    def reporter(self, args: list):
        if args[1] is None:
            del args[1]
        return REPORTER_BLOCKS[args[0]](*args[1:])

    def nott(self, args: list):
        return gm.Not(args[0])

    def minus(self, args: list):
        return gm.Sub(0, args[0])

    def andd(self, args: list):
        return gm.And(args[0], args[1])

    def orr(self, args: list):
        return gm.Or(args[0], args[1])

    def eq(self, args: list):
        return gm.Eq(args[0], args[1])

    def lt(self, args: list):
        return gm.Lt(args[0], args[1])

    def gt(self, args: list):
        return gm.Gt(args[0], args[1])

    def add(self, args: list):
        return gm.Add(args[0], args[1])

    def sub(self, args: list):
        return gm.Sub(args[0], args[1])

    def join(self, args: list):
        return gm.Join(args[0], args[1])

    def mul(self, args: list):
        return gm.Mul(args[0], args[1])

    def div(self, args: list):
        return gm.Div(args[0], args[1])

    def mod(self, args: list):
        return gm.Mod(args[0], args[1])

    def ofreporter(self, args: list):
        if args[2] is None:
            del args[2]
        return getattr(args[0], args[1])(*args[2:])

    def ofstatement(self, args: list):
        if args[2] is None:
            del args[2]
        return getattr(args[0], args[1])(*args[2:])

    def listget(self, args: list):
        return args[0][args[1]]

    def forever(self, args: list):
        return gm.Forever(*args[0])

    def repeat(self, args: list):
        return gm.Repeat(args[0])(*args[1])

    def until(self, args: list):
        return gm.Until(args[0])(*args[1])

    def iff(self, args: list):
        return gm.If(args[0])(*args[1])

    def ifelse(self, args: list):
        return gm.If(args[0])(*args[1]).Else(*args[2])

    def ifelseif(self, args: list):
        condition = args[0]
        if_stack = args[1]
        blk1 = gm.If(condition)(*if_stack)
        blk = blk1
        it = iter(args[2:])
        for condition, if_stack in zip(it, it):
            blk2 = gm.If(condition)(*if_stack)
            blk.Else(blk2)
            blk = blk2
        return blk1

    def ifelseifelse(self, args: list):
        condition = args[0]
        if_stack = args[1]
        blk1 = gm.If(condition)(*if_stack)
        blk = blk1
        it = iter(args[2:-1])
        for condition, if_stack in zip(it, it):
            blk2 = gm.If(condition)(*if_stack)
            blk.Else(blk2)
            blk = blk2
        blk.Else(*args[-1])
        return blk1

    def varchange(self, args: list):
        return args[0].change(args[1])

    def listchange(self, args: list):
        return args[0].replace(args[1], args[2])
