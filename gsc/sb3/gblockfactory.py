import importlib.resources
from typing import NamedTuple


class gPrototype(NamedTuple):
    name: str
    arguments: list[str]
    opcode: str


def load_prototypes(file: str):
    prototypes: dict[str, gPrototype] = {}
    i = importlib.resources.open_text("res", file)
    next(i)
    next(i)
    for line in i:
        name, arguments, opcode = [i.strip() for i in line.strip(" \n").split("|")]
        arguments = [i.strip() for i in arguments.split(",")]
        if arguments[0] == "":
            arguments = []
        prototypes[name] = gPrototype(name, arguments, opcode)
    return prototypes


reporter_prototypes = load_prototypes("reporters.txt")
statement_prototypes = load_prototypes("statements.txt")
hat_prototypes = load_prototypes("hats.txt")
