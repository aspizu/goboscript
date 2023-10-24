from __future__ import annotations
from typing import NamedTuple
from importlib.resources import files
from .. import res


class Prototype(NamedTuple):
    name: str
    arguments: list[str]
    opcode: str


def load_prototypes(file: str):
    prototypes: dict[str, Prototype] = {}
    i = (files(res) / file).open()
    for line in i:
        if line.strip().startswith("#") or line.strip() == "":
            continue
        name, arguments, opcode = (i.strip() for i in line.strip(" \n").split("|"))
        arguments = [i.strip() for i in arguments.split(",")]
        if arguments[0] == "":
            arguments = []
        prototypes[name.removesuffix("?")] = Prototype(name, arguments, opcode)
    return prototypes


reporter_prototypes = load_prototypes("reporters.txt")
statement_prototypes = load_prototypes("statements.txt")
