from difflib import get_close_matches
from itertools import chain
from typing import Any

from gdefinitionvisitor import gDefinitionVisitor
from gerror import gTokenError
from gparser import literal
from lark import Token, Transformer
from lib import num_plural
from sb3 import gBlock, gHatBlock, gInputType, gProcCall, gStack
from sb3.gblockfactory import hat_prototypes, reporter_prototypes, statement_prototypes


class gBlockTransformer(Transformer[Token, gBlock]):
    def __init__(
        self, gdefinitionvisitor: gDefinitionVisitor, visit_tokens: bool = True
    ):
        super().__init__(visit_tokens)
        self.gdefinitionvisitor = gdefinitionvisitor

    def STRING(self, token: Token):
        return literal(token)

    NUMBER = STRING
    FLOAT = STRING

    def start(self, args: tuple[gBlock]):
        return args[0]

    def expr(self, args: tuple[gInputType]):
        return args[0]

    def declr_hat(self, args: list[Any]) -> gHatBlock:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:-1]
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
        return gStack(args)

    def block(self, args: list[Any]) -> gBlock:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:]
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
                    f"Missing arguments: {', '.join(prototype.arguments[len(arguments):])}",
                )
            return gBlock.from_prototype(prototype, arguments)
        elif opcode in self.gdefinitionvisitor.functions.keys():
            argument_names = self.gdefinitionvisitor.functions[opcode]
            return gProcCall(opcode, dict(zip(argument_names, arguments)), False)
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

    def reporter(self, args: list[Any]) -> gBlock:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:]
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
                f"Missing arguments: {', '.join(prototype.arguments[len(arguments):])}",
            )
        return gBlock.from_prototype(prototype, arguments)
