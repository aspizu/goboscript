from __future__ import annotations

import json
from collections.abc import Iterable
from dataclasses import dataclass
from typing import TextIO


@dataclass
class Prototype:
    name: str
    arguments: list[str]
    opcode: str


def load_prototypes(file: TextIO):
    prototypes: dict[str, Prototype] = {}
    for line in file:
        if line.strip().startswith("#") or line.strip() == "":
            continue
        name, arguments, opcode = (i.strip() for i in line.strip(" \n").split("|"))
        arguments = [i.strip() for i in arguments.split(",")]
        if arguments[0] == "":
            arguments = []
        prototypes[name.removesuffix("?")] = Prototype(name, arguments, opcode)
    return prototypes


reporter_prototypes = load_prototypes(open("reporters.txt"))
statement_prototypes = load_prototypes(open("statements.txt"))
en = json.load(open("en.json"))


def fmt(format: str, values: Iterable[str]) -> str:
    for i, value in enumerate(values):
        format = format.replace(f"%{i+1}", f"({value}:: custom)")
    return format


with open("src/statement-blocks.md", "w") as f:
    f.write("# Statement Blocks\n")
    for func in statement_prototypes.values():
        proccode = en[func.opcode.upper()]
        gs = func.name
        args = [arg.lower() for arg in func.arguments]
        if func.arguments:
            gs += " " + ", ".join(args)
        gs += ";"
        if func.name == "waituntil":
            blocks = "wait until <condition:: custom>"
        else:
            blocks = fmt(proccode, args)
        f.write(f"```goboscript\n{gs}\n```\n")
        f.write(f"```blocks\n{blocks}\n```\n")
    f.write("This file was auto-generated.\n")

with open("src/reporter-blocks.md", "w") as f:
    f.write("# Reporter Blocks\n")
    for func in reporter_prototypes.values():
        proccode = en[func.opcode.upper()]
        gs = func.name.removesuffix("?") + "("
        args = [arg.lower() for arg in func.arguments]
        if func.arguments:
            gs += ", ".join(args)
        gs += ")"
        if not func.name.endswith("?"):
            blocks = "(" + fmt(proccode, args) + ")"
        else:
            blocks = "<" + fmt(proccode, args) + ">"
        f.write(f"```goboscript\n{gs}\n```\n")
        f.write(f"```blocks\n{blocks}\n```\n")
    f.write("This file was auto-generated.\n")
