import math
from difflib import get_close_matches
from itertools import chain
from typing import Any, Iterator, cast

from gdefinitionvisitor import gDefinitionVisitor, gFunction
from gerror import gFileError, gTokenError
from gparser import literal
from lark import Token, Transformer
from lib import num_plural, number
from sb3 import (
    gArgument,
    gBlock,
    gHatBlock,
    gInputType,
    gList,
    gProcCall,
    gProcDef,
    gStack,
    gVariable,
)
from sb3.gblockfactory import hat_prototypes, reporter_prototypes, statement_prototypes


def mkelif(
    rest: Iterator[tuple[gInputType, gStack]],
    this: tuple[gInputType, gStack] | None = None,
    _else: gStack | None = None,
) -> gBlock:
    if this is None:
        this = next(rest)
    try:
        nxt = next(rest)
    except StopIteration:
        if _else is None:
            return gBlock("control_if", {"CONDITION": this[0], "SUBSTACK": this[1]}, {})
        else:
            return gBlock(
                "control_if_else",
                {"CONDITION": this[0], "SUBSTACK": this[1], "SUBSTACK2": _else},
                {},
            )
    return gBlock(
        "control_if_else",
        {
            "CONDITION": this[0],
            "SUBSTACK": this[1],
            "SUBSTACK2": gStack([mkelif(rest, nxt, _else)]),
        },
        {},
    )


class gBlockTransformer(Transformer[Token, gBlock]):
    def __init__(
        self,
        gdefinitionvisitor: gDefinitionVisitor,
        prototype: gFunction | None = None,
    ):
        super().__init__(True)
        self.gdefinitionvisitor = gdefinitionvisitor
        self.sprite = gdefinitionvisitor.sprite
        self.prototype = prototype

    def STRING(self, token: Token):
        return literal(token)

    NUMBER = STRING
    FLOAT = STRING

    def start(self, args: tuple[gBlock]):
        return args[0]

    def expr(self, args: tuple[gInputType]):
        return args[0]

    def argument(self, args: tuple[Token]):
        if self.prototype:
            argument = literal(args[0])
            if argument not in self.prototype.arguments:
                matches = get_close_matches(argument, self.prototype.arguments)
                raise gTokenError(
                    "Undefined function argument",
                    args[0],
                    f"Did you mean `${matches[0]}`?" if matches else None,
                )
            return gArgument(argument)
        else:
            raise gTokenError(
                "Argument reporter used outsite function declaration", args[0]
            )

    def declr_function(self, args: list[Any], warp: bool = True) -> gProcDef:
        name: Token = args[0]
        arguments: list[Token] = args[1:-1]
        if arguments == [None]:
            arguments = []
        stack: gStack = args[-1]
        return gProcDef(name, arguments, warp, stack)

    def declr_function_nowarp(self, args: list[Any]) -> gProcDef:
        return self.declr_function(args, False)

    def declr_hat(self, args: list[Any]) -> gHatBlock:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:-1]
        if arguments == [None]:
            arguments = []
        stack: gStack = args[-1]
        if opcode not in hat_prototypes:
            matches = get_close_matches(opcode, hat_prototypes.keys())
            raise gTokenError(
                f"Undefined hat `{opcode}`",
                opcode,
                (f"Did you mean `{matches[0]}`?\n" if matches else "")
                + "Read --doc hats for available hats",
            )
        prototype = hat_prototypes[opcode]
        return gHatBlock.from_prototype(prototype, arguments, stack)

    def stack(self, args: list[gBlock]) -> gStack:
        stack = gStack(args)
        for i, block in enumerate(stack):
            if block.opcode == "control_forever" and (i + 1) != len(stack):
                raise gFileError(
                    "forever cannot be precceded by any statements"
                )  # FIXME: switch to gTokenError but cannot because
        return stack

    def block(self, args: list[Any]) -> gBlock:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:]
        if arguments == [None]:
            arguments = []
        if opcode in statement_prototypes:
            prototype = statement_prototypes[opcode]
            if len(arguments) > len(prototype.arguments):
                raise gTokenError(
                    "Too many arguments for statement",
                    opcode,
                    f"Expected {num_plural(len(prototype.arguments), ' argument')}",
                )
            if len(arguments) < len(prototype.arguments):
                raise gTokenError(
                    "Missing arguments for statement",
                    opcode,
                    f"Missing {', '.join(prototype.arguments[len(arguments):])}",
                )
            return gBlock.from_prototype(prototype, arguments)
        elif opcode in self.gdefinitionvisitor.functions.keys():
            prototype = self.gdefinitionvisitor.functions[opcode]
            if len(arguments) > len(prototype.arguments):
                raise gTokenError(
                    "Too many arguments for function",
                    opcode,
                    f"Expected {num_plural(len(prototype), ' argument')}",
                )
            if len(arguments) < len(prototype.arguments):
                raise gTokenError(
                    "Missing arguments for function",
                    opcode,
                    f"Missing {', '.join(prototype.arguments[len(arguments):])}",
                )
            return gProcCall(
                opcode, dict(zip(prototype.arguments, arguments)), prototype.warp
            )
        else:
            matches = get_close_matches(
                args[0],
                chain(
                    statement_prototypes.keys(),
                    self.gdefinitionvisitor.functions.keys(),
                ),
            )
            raise gTokenError(
                f"Undefined statement or function `{opcode}`",
                args[0],
                (f"Did you mean `{matches[0]}`?\n" if matches else "")
                + "Read --doc statements for available statements",
            )

    def reporter(self, args: list[Any]) -> gInputType:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:]
        if arguments == [None]:
            arguments = []
        if opcode not in reporter_prototypes:
            matches = get_close_matches(args[0], reporter_prototypes.keys())
            raise gTokenError(
                f"Undefined reporter `{opcode}`",
                args[0],
                (f"Did you mean `{matches[0]}`?\n" if matches else "")
                + "Read --doc reporters for available reporters",
            )
        prototype = reporter_prototypes[opcode]
        if len(arguments) > len(prototype.arguments):
            raise gTokenError(
                "Too many arguments for reporter",
                opcode,
                f"Expected {num_plural(len(prototype.arguments), ' argument')}",
            )
        if len(arguments) < len(prototype.arguments):
            raise gTokenError(
                "Missing arguments for reporter",
                opcode,
                f"Missing {', '.join(prototype.arguments[len(arguments):])}",
            )
        if (
            prototype.opcode == "operator_mathop.OPERATOR=sqrt"
            and type(arguments[0]) is str
        ):
            return str(math.sqrt(number(arguments[0])))
        return gBlock.from_prototype(prototype, arguments)

    def add(self, args: list[gInputType]):
        if type(args[0]) is str and type(args[1]) is str:
            return str(number(args[0]) + number(args[1]))
        return gBlock.from_prototype(reporter_prototypes["add"], args)

    def sub(self, args: list[gInputType]):
        if type(args[0]) is str and type(args[1]) is str:
            return str(number(args[0]) - number(args[1]))
        return gBlock.from_prototype(reporter_prototypes["sub"], args)

    def minus(self, args: tuple[gInputType]):
        return gBlock.from_prototype(reporter_prototypes["sub"], ["0", args[0]])

    def mul(self, args: list[gInputType]):
        if type(args[0]) is str and type(args[1]) is str:
            return str(number(args[0]) * number(args[1]))
        return gBlock.from_prototype(reporter_prototypes["mul"], args)

    def div(self, args: list[gInputType]):
        if type(args[0]) is str and type(args[1]) is str:
            return str(number(args[0]) / number(args[1]))
        return gBlock.from_prototype(reporter_prototypes["div"], args)

    def mod(self, args: list[gInputType]):
        if type(args[0]) is str and type(args[1]) is str:
            return str(number(args[0]) % number(args[1]))
        return gBlock.from_prototype(reporter_prototypes["mod"], args)

    def join(self, args: list[gInputType]):
        if type(args[0]) is str and type(args[1]) is str:
            return args[0] + args[1]
        return gBlock.from_prototype(reporter_prototypes["join"], args)

    def eq(self, args: list[gInputType]):
        return gBlock.from_prototype(reporter_prototypes["eq"], args)

    def gt(self, args: list[gInputType]):
        return gBlock.from_prototype(reporter_prototypes["gt"], args)

    def lt(self, args: list[gInputType]):
        return gBlock.from_prototype(reporter_prototypes["lt"], args)

    def andop(self, args: list[gInputType]):
        return gBlock.from_prototype(reporter_prototypes["and"], args)

    def orop(self, args: list[gInputType]):
        return gBlock.from_prototype(reporter_prototypes["or"], args)

    def notop(self, args: list[gInputType]):
        return gBlock.from_prototype(reporter_prototypes["not"], args)

    def block_if(self, args: tuple[gInputType, gStack]):
        return gBlock("control_if", {"CONDITION": args[0], "SUBSTACK": args[1]}, {})

    def block_if_else(self, args: tuple[gInputType, gStack, gStack]):
        return gBlock(
            "control_if_else",
            {"CONDITION": args[0], "SUBSTACK": args[1], "SUBSTACK2": args[2]},
            {},
        )

    def block_if_elif(self, args: list[Any]):
        return mkelif(cast(Any, zip(*[iter(args)] * 2)))

    def block_if_elif_else(self, args: list[Any]):
        return mkelif(cast(Any, zip(*[iter(args[:-1])] * 2)), _else=args[-1])

    def until(self, args: tuple[gInputType, gStack]):
        return gBlock(
            "control_repeat_until", {"CONDITION": args[0], "SUBSTACK": args[1]}, {}
        )

    def repeat(self, args: tuple[gInputType, gStack]):
        return gBlock("control_repeat", {"TIMES": args[0], "SUBSTACK": args[1]}, {})

    def forever(self, args: tuple[gStack]):
        return gBlock("control_forever", {"SUBSTACK": args[0]}, {})

    def varset(self, args: tuple[Token, gInputType]):
        return gBlock("data_setvariableto", {"VALUE": args[1]}, {"VARIABLE": args[0]})

    def var(self, args: tuple[Token]) -> gVariable:
        if (
            gVariable(args[0]) not in self.sprite.variables
            and args[0] not in self.gdefinitionvisitor.globals
        ):
            matches = get_close_matches(
                args[0], self.sprite.variables + self.gdefinitionvisitor.globals  # type: ignore
            )
            raise gTokenError(
                f"Undefined variable `{args[0]}`",
                args[0],
                f"Did you mean `{matches[0]}?`" if matches else None,
            )
        return gVariable(args[0])

    def listset(self, args: tuple[Token]):
        return gBlock("data_deletealloflist", {}, {"LIST": args[0]})

    def islist(self, token: Token):
        if (
            gList(token) not in self.sprite.lists
            and token not in self.gdefinitionvisitor.listglobals
        ):
            matches = get_close_matches(
                token, self.sprite.lists + self.gdefinitionvisitor.listglobals  # type: ignore
            )
            raise gTokenError(
                f"Undefined list `{token}`",
                token,
                f"Did you mean `{matches[0]}`?" if matches else None,
            )

    def listadd(self, args: tuple[Token, gInputType]):
        self.islist(args[0])
        return gBlock("data_addtolist", {"ITEM": args[1]}, {"LIST": args[0]})

    def listdelete(self, args: tuple[Token, gInputType]):
        self.islist(args[0])
        return gBlock("data_deleteoflist", {"INDEX": args[1]}, {"LIST": args[0]})

    def listinsert(self, args: tuple[Token, gInputType, gInputType]):
        self.islist(args[0])
        return gBlock(
            "data_insertatlist", {"INDEX": args[1], "ITEM": args[2]}, {"LIST": args[0]}
        )

    def listreplace(self, args: tuple[Token, gInputType, gInputType]):
        self.islist(args[0])
        return gBlock(
            "data_replaceitemoflist",
            {"INDEX": args[1], "ITEM": args[2]},
            {"LIST": args[0]},
        )

    def listshow(self, args: tuple[Token]):
        self.islist(args[0])
        return gBlock("data_showlist", {}, {"LIST": args[0]})

    def listhide(self, args: tuple[Token]):
        self.islist(args[0])
        return gBlock("data_hidelist", {}, {"LIST": args[0]})

    def listitem(self, args: tuple[Token, gInputType]):
        self.islist(args[0])
        return gBlock("data_itemoflist", {"INDEX": args[1]}, {"LIST": args[0]})

    def listindex(self, args: tuple[Token, gInputType]):
        self.islist(args[0])
        return gBlock("data_itemnumoflist", {"ITEM": args[1]}, {"LIST": args[0]})

    def listcontains(self, args: tuple[Token, gInputType]):
        self.islist(args[0])
        return gBlock("data_listcontainsitem", {"ITEM": args[1]}, {"LIST": args[0]})

    def listlength(self, args: tuple[Token]):
        self.islist(args[0])
        return gBlock("data_lengthoflist", {}, {"LIST": args[0]})

    def declr_on(self, args: tuple[Token, gStack]):
        return gHatBlock(
            "event_whenbroadcastreceived",
            {},
            {"BROADCAST_OPTION": [args[0], args[0]]},
            args[1],
        )

    def nop(self, args: tuple[()]):
        return gBlock("control_wait", {"DURATION": "0"}, {})
