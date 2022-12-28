from difflib import get_close_matches
from typing import Any

from gerror import gTokenError
from gparser import literal
from lark import Token, Transformer
from sb3 import gBlock, gHatBlock, gInputType, gStack
from sb3.gblockfactory import (
    hat_prototypes,
    new_hat,
    new_reporter,
    new_statement,
    reporter_prototypes,
    statement_prototypes,
)


class gBlockTransformer(Transformer[Token, gBlock]):
    def STRING(self, token: Token):
        return literal(token)

    def start(self, args: tuple[gBlock]):
        return args[0]

    def expr(self, args: tuple[gInputType]):
        return args[0]

    def declr_hat(self, args: tuple[Token, gStack]) -> gHatBlock:
        if args[0] not in hat_prototypes:
            matches = get_close_matches(args[0], hat_prototypes.keys())
            raise gTokenError(
                f"Undefined hat block `{args[0]}`",
                args[0],
                (f"Did you mean `{matches[0]}`?\n" if matches else "")
                + "Read --doc hat-blocks for available hat blocks",
            )
        return new_hat(args[0], [], args[1])

    def stack(self, args: tuple[gBlock, ...]) -> gStack:
        return gStack(args)

    def block(self, args: list[Any]) -> gBlock:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:]
        if opcode not in statement_prototypes:
            matches = get_close_matches(args[0], statement_prototypes.keys())
            raise gTokenError(
                f"Undefined statement block `{opcode}`",
                args[0],
                (f"Did you mean `{matches[0]}`?\n" if matches else "")
                + "Read --doc statement-blocks for available statement blocks",
            )
        return new_statement(opcode, arguments)

    def reporter(self, args: list[Any]) -> gBlock:
        opcode: Token = args[0]
        arguments: list[gInputType] = args[1:]
        if opcode not in reporter_prototypes:
            matches = get_close_matches(args[0], reporter_prototypes.keys())
            raise gTokenError(
                f"Undefined reporter block `{opcode}`",
                args[0],
                (f"Did you mean `{matches[0]}`?\n" if matches else "")
                + "Read --doc reporter-blocks for available reporter blocks",
            )
        return new_reporter(opcode, arguments)
