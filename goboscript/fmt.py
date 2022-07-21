from pathlib import Path
from parser import parser
from lark.visitors import Interpreter
from lark.tree import Tree


def fmt_all(pathdir: Path) -> None:
    for i in pathdir.glob("**/*.gs"):
        fmt(i)


def fmt(file_pth: Path) -> None:
    tree = parser.parse(file_pth.read_text())
    fmted = Fmt().visit(tree)
    with file_pth.open("w") as f:
        f.write(fmted)


class Fmt(Interpreter):
    def __init__(self):
        self.max = 80
        self.tab = "    "
        self.level = 0

    def strexpr(self, i):
        return self.visit(i) if isinstance(i, Tree) else ("" if i is None else str(i))

    def strexprlist(self, exprlist, split=False):
        if split:
            return f"\n{self.tab}" + f",\n{self.tab}".join(
                [self.strexpr(i) for i in exprlist]
            )
        else:
            return ", ".join([self.strexpr(i) for i in exprlist])

    def indent(self, text):
        return f"\n{self.tab}".join(str(text).split("\n"))

    def start(self, tree):
        return "\n\n".join(self.visit_children(tree))

    def costumes(self, tree):
        a = "costumes " + ", ".join(tree.children) + ";\n"
        if len(a) > self.max:
            a = f"costumes\n{self.tab}" + f",\n{self.tab}".join(tree.children) + ";"
        return a

    def sounds(self, tree):
        a = "sounds " + ", ".join(tree.children) + ";\n"
        if len(a) > self.max:
            a = f"sounds\n{self.tab}" + f",\n{self.tab}".join(tree.children) + ";"
        return a

    def use(self, tree):
        return "use " + tree.children[0] + ";"

    def hat(self, tree):
        return (
            tree.children[0]
            + ("" if tree.children[1] is None else " ")
            + self.strexprlist(tree.children[1:-1])
            + " "
            + self.visit(tree.children[-1])
        )

    def stack(self, tree):
        self.level += 1
        v1 = "{"
        v2 = self.indent("\n" + "\n".join(self.visit_children(tree)))
        v3 = "\n}"
        self.level -= 1
        return v1 + v2 + v3

    def statement(self, tree):
        a = (
            tree.children[0]
            + ("" if tree.children[1] is None else " ")
            + self.strexprlist(tree.children[1:])
            + ";"
        )
        if len(self.level * self.tab + a) > self.max:
            a = (
                tree.children[0]
                + ("" if tree.children[1] is None else " ")
                + self.strexprlist(tree.children[1:], split=True)
                + ";"
            )
        return a

    def ofstatement(self, tree):
        a = (
            tree.children[0]
            + "."
            + tree.children[1]
            + ("" if tree.children[1] is None else " ")
            + self.strexprlist(tree.children[2:])
            + ";"
        )
        if len(self.level * self.tab + a) > self.max:
            a = (
                tree.children[0]
                + "."
                + tree.children[1]
                + ("" if tree.children[1] is None else " ")
                + self.strexprlist(tree.children[2:], split=True)
                + ";"
            )
        return a

    def gofstatement(self, tree):
        a = (
            "$"
            + tree.children[0]
            + "."
            + tree.children[1]
            + ("" if tree.children[1] is None else " ")
            + self.strexprlist(tree.children[2:])
            + ";"
        )
        if len(self.level * self.tab + a) > self.max:
            a = (
                "$"
                + tree.children[0]
                + "."
                + tree.children[1]
                + ("" if tree.children[1] is None else " ")
                + self.strexprlist(tree.children[2:], split=True)
                + ";"
            )
        return a

    def parenexpr(self, tree):
        return "(" + self.strexpr(tree.children[0]) + ")"

    def reporter(self, tree):
        return tree.children[0] + "(" + self.strexpr(tree.children[1]) + ")"

    def ofreporter(self, tree):
        return (
            tree.children[0]
            + "."
            + tree.children[1]
            + "("
            + self.strexpr(tree.children[2])
            + ")"
        )

    def gofreporter(self, tree):
        return (
            "$"
            + tree.children[0]
            + "."
            + tree.children[1]
            + "("
            + self.strexpr(tree.children[2])
            + ")"
        )

    def var(self, tree):
        return tree.children[0]

    def gvar(self, tree):
        return "$" + tree.children[0]

    def argument(self, tree):
        return "@" + tree.children[0]

    def varset(self, tree):
        return tree.children[0] + " = " + self.strexpr(tree.children[1]) + ";"

    def gvarset(self, tree):
        return "$" + tree.children[0] + " = " + self.strexpr(tree.children[1]) + ";"

    def iff(self, tree):
        return (
            "if " + self.strexpr(tree.children[0]) + " " + self.visit(tree.children[1])
        )

    def repeat(self, tree):
        return (
            "repeat "
            + self.strexpr(tree.children[0])
            + " "
            + self.visit(tree.children[1])
        )

    def until(self, tree):
        return (
            "until "
            + self.strexpr(tree.children[0])
            + " "
            + self.visit(tree.children[1])
        )

    def ifelse(self, tree):
        return (
            "if "
            + self.strexpr(tree.children[0])
            + " "
            + self.visit(tree.children[1])
            + " else "
            + self.visit(tree.children[2])
        )

    def varchange(self, tree):
        return tree.children[0] + " += " + self.strexpr(tree.children[1]) + ";"

    def varsub(self, tree):
        return tree.children[0] + " -= " + self.strexpr(tree.children[1]) + ";"

    def varmul(self, tree):
        return tree.children[0] + " *= " + self.strexpr(tree.children[1]) + ";"

    def vardiv(self, tree):
        return tree.children[0] + " /= " + self.strexpr(tree.children[1]) + ";"

    def varmod(self, tree):
        return tree.children[0] + " %= " + self.strexpr(tree.children[1]) + ";"

    def varjoin(self, tree):
        return tree.children[0] + " ++= " + self.strexpr(tree.children[1]) + ";"

    def gvarchange(self, tree):
        return "$" + tree.children[0] + " += " + self.strexpr(tree.children[1]) + ";"

    def gvarsub(self, tree):
        return "$" + tree.children[0] + " -= " + self.strexpr(tree.children[1]) + ";"

    def gvarmul(self, tree):
        return "$" + tree.children[0] + " *= " + self.strexpr(tree.children[1]) + ";"

    def gvardiv(self, tree):
        return "$" + tree.children[0] + " /= " + self.strexpr(tree.children[1]) + ";"

    def gvarmod(self, tree):
        return "$" + tree.children[0] + " %= " + self.strexpr(tree.children[1]) + ";"

    def gvarjoin(self, tree):
        return "$" + tree.children[0] + " ++= " + self.strexpr(tree.children[1]) + ";"

    def lstset(self, tree):
        return tree.children[0] + " = [];"

    def lstchange(self, tree):
        return (
            tree.children[0]
            + "["
            + tree.children[1]
            + "] = "
            + self.strexpr(tree.children[1])
            + ";"
        )

    def glstset(self, tree):
        return "$" + tree.children[0] + " = [];"

    def glstchange(self, tree):
        return (
            "$"
            + tree.children[0]
            + "["
            + tree.children[1]
            + "] = "
            + self.strexpr(tree.children[1])
            + ";"
        )

    def lstitem(self, tree):
        return tree.children[0] + "[" + self.strexpr(tree.children[1]) + "]"

    def glstitem(self, tree):
        return "$" + tree.children[0] + "[" + self.strexpr(tree.children[1]) + "]"

    def eq(self, tree):
        return self.strexpr(tree.children[0]) + " = " + self.strexpr(tree.children[1])

    def lt(self, tree):
        return self.strexpr(tree.children[0]) + " < " + self.strexpr(tree.children[1])

    def gt(self, tree):
        return self.strexpr(tree.children[0]) + " > " + self.strexpr(tree.children[1])

    def andd(self, tree):
        return self.strexpr(tree.children[0]) + " & " + self.strexpr(tree.children[1])

    def orr(self, tree):
        return self.strexpr(tree.children[0]) + " | " + self.strexpr(tree.children[1])

    def nott(self, tree):
        return "!" + self.strexpr(tree.children[0])

    def minus(self, tree):
        return "-" + self.strexpr(tree.children[0])

    def add(self, tree):
        return self.strexpr(tree.children[0]) + " + " + self.strexpr(tree.children[1])

    def sub(self, tree):
        return self.strexpr(tree.children[0]) + " - " + self.strexpr(tree.children[1])

    def mul(self, tree):
        return self.strexpr(tree.children[0]) + " * " + self.strexpr(tree.children[1])

    def div(self, tree):
        return self.strexpr(tree.children[0]) + " / " + self.strexpr(tree.children[1])

    def mod(self, tree):
        return self.strexpr(tree.children[0]) + " % " + self.strexpr(tree.children[1])

    def join(self, tree):
        return self.strexpr(tree.children[0]) + " ++ " + self.strexpr(tree.children[1])

    def deff(self, tree):
        a = (
            ("" if tree.children[0] is None else "nowarp")
            + "def "
            + tree.children[1]
            + ("" if tree.children[2] is None else " ")
            + ", ".join(tree.children[2:-1])
            + " "
            + self.visit(tree.children[-1])
        )
        if len(a) > self.max:
            a = (
                ("" if tree.children[0] is None else "nowarp")
                + "def "
                + tree.children[1]
                + ("" if tree.children[2] is None else " ")
                + "\n"
                + self.tab
                + f",\n{self.tab}".join(tree.children[2:-1])
                + "\n"
                + self.visit(tree.children[-1])
            )
        return a
