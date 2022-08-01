from pathlib import Path

import gobomatic as gm
import lark

import blocks
from common import STDLIB_PTH, CompilerFileError, Token_STRING_to_str, gm_bool


class CollectDefinitions(lark.visitors.Visitor):

    def __init__(self, sprite: gm.Sprite, local_vars: dict[str, gm.Var],
                 local_lsts: dict[str, gm.List]):
        self.sprite = sprite
        self.local_vars = local_vars
        self.local_lsts = local_lsts

    def varset(self, tree):
        var = str(tree.children[0].children[0])
        if var[0] == ".":
            return
        if var[0] == "$":
            return
        if var not in self.local_vars:
            self.local_vars[var] = self.sprite.Var(var)

    def lstset(self, tree):
        lst = str(tree.children[0].children[0])
        if lst[0] == ".":
            return
        if lst[0] == "$":
            return
        if lst not in self.local_lsts:
            self.local_lsts[lst] = self.sprite.List(lst)


class Transformer(lark.visitors.Transformer):

    def __init__(self,
                 sprite: gm.Sprite,
                 local_vars: dict[str, gm.Var],
                 local_lsts: dict[str, gm.List],
                 funcs: dict[str, gm.Sprite.FuncFactory],
                 args: set[str],
                 namespace: str = None):
        self.sprite = sprite
        self.local_vars = local_vars
        self.local_lsts = local_lsts
        self.funcs = funcs
        self.args = args
        self.namespace = namespace
        super().__init__()

    def NUMBER(self, token):
        return str(token)

    def BOOL(self, token):
        return 1 if token == "true" else 0

    def STRING(self, token):
        return Token_STRING_to_str(token)

    def stack(self, args):
        return args or [gm.StopThisScript()]

    def exprgroup(self, args):
        return args[0]

    def variable(self, args):
        var = str(args[0])
        try:
            if var[0] == "$":
                return gm.Var(var)
            elif var[0] == "@":
                if var not in self.args:
                    raise CompilerFileError(args[0],
                                            f"Undefined argument '{args[0]}'")
                return getattr(gm.Arg, var)
            elif var[0] == ".":
                if self.namespace:
                    if not self.namespace + var in self.local_vars:
                        self.local_vars[self.namespace +
                                        var] = self.sprite.Var(self.namespace +
                                                               var)
                    return self.local_vars[self.namespace + var]
                else:
                    raise CompilerFileError(
                        args[0], "Namespace variables cannot be used here")
            else:
                if var in self.local_vars:
                    return self.local_vars[var]
                else:
                    return self.local_lsts[var]
        except KeyError:
            raise CompilerFileError(args[0], f"Undefined variable '{args[0]}'")

    def statement(self, args):
        name: lark.Token = args[0]
        args = args[1:] if args[1] is not None else []
        try:
            try:
                return blocks.STATEMENT[name](*args)
            except KeyError:
                return self.funcs[name](*args)
        except TypeError as ex:
            raise CompilerFileError(name, ex.args[0])
        except KeyError:
            raise CompilerFileError(name, f"Invalid statement block '{name}'")

    def ofstatement(self, args):
        of = args[0]
        name = args[1]
        args = args[2:] if args[2] else []
        try:
            return getattr(of, name)(*args)
        except TypeError as ex:
            raise CompilerFileError(name, ex.args[0])
        except AttributeError as ex:
            if "has no attribute" in ex.args[0]:
                raise CompilerFileError(name,
                                        f"Invalid statement block '{name}'")

    def ofreporter(self, args):
        of = args[0]
        name = args[1]
        args = args[2:] if args[2] else []
        try:
            return getattr(of, name)(*args)
        except TypeError as ex:
            raise CompilerFileError(name, ex.args[0])
        except AttributeError as ex:
            if "has no attribute" in ex.args[0]:
                raise CompilerFileError(name,
                                        f"Invalid reporter block '{name}'")

    def reporter(self, args):
        name: lark.Token = args[0]
        args = args[1:] if args[1] is not None else []
        try:
            return blocks.REPORTER[name](*args)
        except TypeError as ex:
            raise CompilerFileError(name,
                                    ex.args[0].split(")")[1][1:].capitalize())
        except KeyError:
            raise CompilerFileError(name, f"Invalid reporter block '{name}'")

    def varset(self, args):
        name: gm.Var = args[0]
        return name.set(args[1])

    def lstset(self, args):
        name: gm.List = args[0]
        return name.delete_all()

    def add(self, args):
        return gm.Add(args[0], args[1])

    def sub(self, args):
        return gm.Sub(args[0], args[1])

    def mul(self, args):
        return gm.Mul(args[0], args[1])

    def div(self, args):
        return gm.Div(args[0], args[1])

    def mod(self, args):
        return gm.Mod(args[0], args[1])

    def join(self, args):
        return gm.Join(args[0], args[1])

    def eq(self, args):
        return gm.Eq(args[0], args[1])

    def lt(self, args):
        return gm.Lt(args[0], args[1])

    def gt(self, args):
        return gm.Gt(args[0], args[1])

    def andd(self, args):
        return gm.And(args[0], args[1])

    def orr(self, args):
        return gm.Orr(args[0], args[1])

    def minus(self, args):
        return gm.Sub(0, args[0])

    def nott(self, args):
        return gm.Not(0, args[0])

    def iff(self, args):
        return gm.If(gm_bool(args[0]))(*args[1])

    def ifelse(self, args):
        return gm.If(gm_bool(args[0]))(*args[1]).Else(*args[2])

    def ifelseif(self, args):
        A = gm.If(gm_bool(args[0]))(*args[1])
        a = A
        i = iter(args[2:])
        for i, j in zip(i, i):
            b = gm.If(gm_bool(i))(*j)
            a.Else(b)
            a = b
        return A

    def ifelseifelse(self, args):
        A = gm.If(args[0])(*args[1])
        a = A
        it = iter(args[2:-1])
        for i, j in zip(it, it):
            b = gm.If(gm_bool(i))(*j)
            a.Else(b)
            a = b
        a.Else(*args[-1])
        return A


class Interpreter(lark.visitors.Interpreter):

    def __init__(self, sprite_pth: Path, sprite: gm.Sprite):
        self.sprite_pth = sprite_pth
        self.sprite = sprite
        self.local_vars: dict[str, gm.Var] = {}
        self.local_lsts: dict[str, gm.List] = {}
        self.funcs: dict[str, gm.Sprite.FuncFactory] = {}
        super().__init__()

    def transform(self, tree, args: set[str], namespace: str = None):
        collector = CollectDefinitions(self.sprite, self.local_vars,
                                       self.local_lsts)
        collector.visit(tree)
        return Transformer(self.sprite, self.local_vars, self.local_lsts,
                           self.funcs, args, namespace).transform(tree)

    start = lark.visitors.Interpreter.visit_children

    def costumes(self, tree):
        self.sprite.costumes = []
        for token in tree.children:
            token: lark.Token
            string = Token_STRING_to_str(token)
            if string.startswith("std:"):
                string = string[len("std:"):]
                costume_pth = STDLIB_PTH / "costumes" / string
                if not costume_pth.is_file():
                    raise CompilerFileError(token,
                                            f"File not in stdlib '{string}'")
                self.sprite.costumes.append(costume_pth.absolute().as_posix())
            else:
                costume_pths = list(self.sprite_pth.parent.glob(string))
                if not costume_pths:
                    raise CompilerFileError(token, "No files found")
                for costume_pth in costume_pths:
                    if not costume_pth.is_file():
                        raise CompilerFileError(
                            token, f"File not found '{costume_pth}'")
                    self.sprite.costumes.append(
                        costume_pth.absolute().as_posix())

    def hat(self, tree):
        name: lark.Token = tree.children[0]
        stack = self.transform(tree.children[-1], set())
        if name == "onflag":
            self.sprite.WhenFlagClicked(*stack)
        else:
            raise CompilerFileError(name, f"Invalid hat block '{name}'")

    def funcdef(self, tree):
        name: lark.Token = tree.children[0]
        args = tree.children[1:-1] if tree.children[1] else []
        stack = tree.children[-1]
        func = self.sprite.Func(*[getattr(gm.Arg, "@" + str(i)) for i in args],
                                name=str(name))
        self.funcs[str(name)] = func
        func.Define(*self.transform(stack, set("@" + str(i)
                                               for i in args), str(name)))
