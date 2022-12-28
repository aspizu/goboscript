from typing import NamedTuple

from .gblock import gBlock, gHatBlock, gInputType, gStack

REPORTER_PROTOTYPES = """
add(NUM1, NUM2)        -> operator_add
sub(NUM1, NUM2)        -> operator_subtract
mul(NUM1, NUM2)        -> operator_multiply
div(NUM1, NUM2)        -> operator_divide
eq(NUM1, NUM2)         -> operator_equals
gt(NUM1, NUM2)         -> operator_gt
lt(NUM1, NUM2)         -> operator_lt
and(NUM1, NUM2)        -> operator_and
or(NUM1, NUM2)         -> operator_or
not(OPERAND)           -> operator_not
join(STRING1, STRING2) -> operator_join
"""

STATEMENT_PROTOTYPES = """
say MESSAGE; -> looks_say
move STEPS;  -> motion_movesteps
"""

HAT_PROTOTYPES = """
onflag { ... }  -> events_whenflagclicked
onclick { ... } -> events_whenspriteclicked
"""


class BlockPrototype(NamedTuple):
    opcode: str
    arguments: list[str]


def magic1(s: str) -> dict[str, BlockPrototype]:
    # fmt: off
    return {p[0]:BlockPrototype(n,p[1])for p,n in(lambda x:[(lambda x,y:((lambda x,y:(x, # type: ignore
           [i.strip()for i in y[:-1].split(",")]))(*x.strip().split("(")),y.strip(),))(* # type: ignore
           i.strip().split("->"))for i in x])(s.strip().split("\n"))}                    # type: ignore
    # fmt: on


def magic2(s: str) -> dict[str, BlockPrototype]:
    # fmt: off
    return {p[0]:BlockPrototype(n,p[1])for p,n in(lambda x:[(lambda x,y:((lambda x,*y:(x # type: ignore
           ,[i.strip(' ,;')for i in y]))(*x.strip().split(" ")),y.strip(),))(*i.strip(). # type: ignore
           split("->"))for i in x])(s.strip().split("\n"))}                              # type: ignore
    # fmt: on


def magic3(s: str) -> dict[str, BlockPrototype]:
    # fmt: off
    return {p[0]:BlockPrototype(n,p[1])for p,n in(lambda x:[(lambda x,y:((lambda x,*y:(x # type: ignore
           ,[i.strip(' ,;')for i in y]))(*x.strip().split(" ")),y.strip(),))(*i.strip(). # type: ignore
           split("->"))for i in x])(s.strip().split("\n"))}                              # type: ignore
    # fmt: on


reporter_prototypes = magic1(REPORTER_PROTOTYPES)
statement_prototypes = magic2(STATEMENT_PROTOTYPES)
hat_prototypes = magic3(HAT_PROTOTYPES)


def new(prototypes: dict[str, BlockPrototype], opcode: str, inputs: list[gInputType]):
    prototype = prototypes[opcode]
    if len(inputs) != len(prototype.arguments):
        raise ValueError(len(inputs))
    return gBlock(prototype.opcode, dict(zip(prototype.arguments, inputs)), {})


def new_statement(opcode: str, inputs: list[gInputType]):
    return new(statement_prototypes, opcode, inputs)


def new_reporter(opcode: str, inputs: list[gInputType]):
    return new(reporter_prototypes, opcode, inputs)


def new_hat(opcode: str, inputs: list[gInputType], stack: gStack):
    prototype = hat_prototypes[opcode]
    return gHatBlock(
        prototype.opcode, dict(zip(prototype.arguments, inputs)), {}, stack
    )
