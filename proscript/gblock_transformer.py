from lark import Transformer  # type: ignore

import blockdefs
from scratch import *
from utils import gTokenException


class gBlockTransformer(Transformer):
    def __init__(self, procs: dict[str, gProcDef]):
        self.procs = procs

    def expr(self, args):
        return args[0]

    def declr_hat(self, args):
        return gHatBlock("event_whenflagclicked", {}, {}, args[2])

    def stack(self, args):
        return tuple(args)

    def block(self, args):
        if args[0] in blockdefs.statement:
            return gBlock(
                blockdefs.statement[args[0]][0],
                dict(zip(blockdefs.statement[args[0]][1], args[1:])),
                {},
            )
        elif args[0] in self.procs:
            proc = self.procs[args[0]]
            return gProcCall(proc.name, {k: v for k, v in zip(proc.args, args[1:])})
        else:
            raise gTokenException(
                "unknown statment block or undefined procedure", args[0]
            )

    def reporter(self, args):
        if args[0] in blockdefs.reporter:
            return gBlock(
                blockdefs.reporter[args[0]][0],
                dict(zip(blockdefs.reporter[args[0]][1], args[1:])),
                {},
            )
        else:
            raise gTokenException("unknown reporter block", args[0])

    def arg(self, args):
        return gArg(str(args[0]))
