from rich import print
from lark import Lark, Transformer
from importlib import resources
import gobomatic as G
import gobomatic.blocks.events as GE


class GobomaticTransformer(Transformer):
    def __init__(self):
        self.variables: dict[str, G.Var] = {}

    def start(self, args):
        return args[0]

    def num(self, args):
        return int(args[0])

    def bool(self, args):
        return args[0] == "true"

    def name(self, args):
        if not args[0] in self.variables:
            self.variables[args[0]] = G.Var(name=str(args[0]))
        return self.variables[args[0]]

    def expr(self, args):
        return args[0]

    def add(self, args):
        return G.Add(args[0], args[2])

    def call(self, args):
        if args[0] == "round":
            return G.Round(args[2])
        return args

    def statement(self, args):
        if args[0] == "say":
            return G.Say(args[2])
        elif args[0] == "sayfor":
            return G.SayFor(args[2], args[4])
        return args
    
    def stack(self, args):
        return G.Stack(args[1:-1])
    
    def onflag(self, args):
        return GE.WhenFlagClicked(args[1])
    
    def string(self, args):
        return str(args[0][1:-1])


gobomatic_parser = Lark(resources.read_text("data", "grammer.lark"), start="start")


ret1 = gobomatic_parser.parse("""
onflag {
    sayfor "variable + 5", 2;
    say "variable + 5";
    say "variable + 5";
}
""")


gbt = GobomaticTransformer()

ret2 = gbt.transform(ret1)

print(ret1)
print(ret2)

#"""
stage = G.Sprite(name="Stage", costumes=["assets/blank.svg"])
main = G.Sprite(name="Main", costumes=["assets/blank.svg"])
main.blocks.append(ret2)

proj = G.Project(sprites=[stage, main])
proj.export("proj.sb3")
#"""
