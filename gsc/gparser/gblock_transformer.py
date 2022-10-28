from lark.lexer import Token
from lark.visitors import Transformer
from sb3 import *
from sb3.gblock import InputType

from . import blockdefs, gsprite_interpreter
from .gexception import *
from .gparser import parse_token


def create_gblock(
    name: str,
    args: list[InputType],
    blockdefs: blockdefs.blockdef_T,
) -> gBlock:
    blk = blockdefs[name]
    return gBlock(blk[0], dict(zip(blk[1], args)), {})


class gBlockTransformer(Transformer):
    def __init__(self, collector: "gsprite_interpreter.DefCollector") -> None:
        super().__init__()
        self.collector = collector

    def declr_proc(self, _args) -> gHatBlock:
        warp: bool = _args[0] is None
        name: Token = _args[1]
        args: list[str] = [str(i) for i in _args[2:-1]]
        stack: gStack = _args[-1]
        return new_proc_def(name, args, warp, stack)

    def declr_hat(self, _args) -> gHatBlock:
        # name: Token = _args[0]
        # args: list[InputType] = _args[1:-1]
        stack: gStack = _args[-1]
        return gHatBlock("event_whenflagclicked", {}, {}, stack)

    def stack(self, _args) -> gStack:
        return gStack(_args)

    def block(self, _args) -> gBlock:
        name: Token = _args[0]
        args: list[InputType] = _args[1:]
        if str(name) in self.collector.procs:
            return gProcCall(str(name), dict(zip(self.collector.procs[name], args)))
        else:
            try:
                return create_gblock(name, args, blockdefs.statement)
            except KeyError:
                raise gCodeError(name, "Undefined statement block")

    def block_setvar(self, _args) -> gBlock:
        return gBlock("data_setvariableto", {"VALUE": _args[1]}, {"VARIABLE": _args[0]})

    def block_changevar(self, _args) -> gBlock:
        if _args[1] == "+":
            return gBlock(
                "data_changevariableby", {"VALUE": _args[2]}, {"VARIABLE": _args[0]}
            )
        else:
            return gBlock(
                "data_setvariableto",
                {
                    "VALUE": create_gblock(
                        str(_args[1]), [_args[0], _args[2]], blockdefs.reporter
                    )
                },
                {"VARIABLE": _args[0]},
            )

    def block_setlist(self, _args) -> gBlock:
        return gBlock("data_deletealloflist", {}, {"LIST": _args[0]})

    def block_setlistitem(self, _args) -> gBlock:
        return gBlock(
            "data_replaceitemoflist",
            {"INDEX": _args[1], "ITEM": _args[2]},
            {"LIST": _args[0]},
        )

    def block_changelistitem(self, _args) -> gBlock:
        return gBlock(
            "data_replaceitemoflist",
            {
                "INDEX": _args[1],
                "VALUE": create_gblock(
                    str(_args[2]),
                    [
                        gBlock(
                            "data_itemoflist", {"INDEX": _args[1]}, {"LIST": _args[0]}
                        )
                    ],
                    blockdefs.statement,
                ),
            },
            {"LIST": _args[0]},
        )

    def block_repeat(self, _args) -> gBlock:
        expr: InputType = _args[0]
        stack: gStack = _args[1]
        return gBlock("control_repeat", {"TIMES": expr, "SUBSTACK": stack}, {})

    def block_until(self, _args) -> gBlock:
        expr: InputType = _args[0]
        stack: gStack = _args[1]
        return gBlock(
            "control_repeat_until", {"CONDITION": expr, "SUBSTACK": stack}, {}
        )

    def block_forever(self, _args) -> gBlock:
        stack: gStack = _args[0]
        return gBlock("control_forever", {"SUBSTACK": stack}, {})

    def block_if(self, _args) -> gBlock:
        condition: InputType = _args[0]
        stack: gStack = _args[1]
        return gBlock("control_if", {"CONDITION": condition, "SUBSTACK": stack}, {})

    def arg(self, _args) -> gBlock:
        return new_arg_reporter(_args[0])

    def var(self, _args) -> gVariable | gList:
        name: Token = _args[0]
        try:
            if name[0] == "$":
                try:
                    return self.collector.global_lists[str(name)]
                except KeyError:
                    return self.collector.global_variables[str(name)]
            else:
                try:
                    return self.collector.lists[str(name)]
                except KeyError:
                    return self.collector.variables[str(name)]
        except KeyError:
            raise gCodeError(name, "Undefined variable/list")

    def reporter(self, _args) -> gBlock:
        name: Token = _args[0]
        args: list[InputType] = _args[1:]
        return create_gblock(name, args, blockdefs.reporter)

    def gnot(self, _args) -> gBlock:
        return create_gblock("not", _args, blockdefs.reporter)

    def gand(self, _args) -> gBlock:
        return create_gblock("and", _args, blockdefs.reporter)

    def gor(self, _args) -> gBlock:
        return create_gblock("or", _args, blockdefs.reporter)

    def gt(self, _args) -> gBlock:
        return create_gblock("gt", _args, blockdefs.reporter)

    def lt(self, _args) -> gBlock:
        return create_gblock("lt", _args, blockdefs.reporter)

    def eq(self, _args) -> gBlock:
        return create_gblock("eq", _args, blockdefs.reporter)

    def add(self, _args) -> gBlock:
        return create_gblock("+", _args, blockdefs.reporter)

    def sub(self, _args) -> gBlock:
        return create_gblock("-", _args, blockdefs.reporter)

    def mul(self, _args) -> gBlock:
        return create_gblock("*", _args, blockdefs.reporter)

    def div(self, _args) -> gBlock:
        return create_gblock("/", _args, blockdefs.reporter)

    def expr(self, _args):
        return parse_token(_args[0])
