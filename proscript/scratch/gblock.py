import json
from dataclasses import dataclass
from typing import Any, Optional, TypeAlias, Union

from .glist import gList
from .gvariable import gVariable

InputType: TypeAlias = Union[str, "gBlock", gVariable, gList, "gStack"]
FieldType: TypeAlias = Union[str, gVariable, gList]


@dataclass
class gBlock:
    opcode: str
    inputs: dict[str, InputType]
    fields: dict[str, FieldType]

    def serialize_inputs(self, blocks):
        def serialize_input(i):
            if isinstance(i, gStack):
                i.serialize(blocks)
                return [2, str(id(i[0]))]
            elif isinstance(i, gBlock):
                return [2, str(id(i))]
            elif isinstance(i, gVariable):
                return [2, [12, i.name, i.name]]
            elif isinstance(i, str):
                return [1, [10, str(i)]]
            else:
                raise TypeError(i)

        return {a: serialize_input(b) for a, b in self.inputs.items()}

    def serialize_fields(self):
        def serialize_field(i):
            if isinstance(i, gVariable):
                return [i.name, i.name]
            elif isinstance(i, str):
                return [str(i), None]
            else:
                raise TypeError

        return {a: serialize_field(b) for a, b in self.fields.items()}

    def _serialize(
        self,
        blocks: dict[str, Any],
        next_id: Optional[str],
        parent_id: Optional[str],
    ):
        blocks[str(id(self))] = {
            "opcode": self.opcode,
            "next": next_id,
            "parent": parent_id,
            "inputs": self.serialize_inputs(blocks),
            "fields": self.serialize_fields(),
            "topLevel": isinstance(self, gHatBlock),
        }
        for child in self.inputs.values():
            if isinstance(child, gBlock):
                child._serialize(blocks, None, str(id(self)))
        return blocks

    def serialize(self):
        return self._serialize({}, None, None)


class gStack(tuple):
    def serialize(self, blocks):
        i = iter(self)
        try:
            cb = next(i)
        except StopIteration:
            return blocks
        try:
            nb = next(i)
        except StopIteration:
            return cb._serialize(blocks, None, None)
        pb, cb = cb, nb
        while True:
            try:
                nb = next(i)
            except StopIteration:
                return cb._serialize(blocks, None, str(id(pb)))
            cb._serialize(blocks, str(id(nb)), str(id(pb)))
            pb, cb = cb, nb


@dataclass
class gHatBlock(gBlock):
    stack: tuple[gBlock, ...]

    def serialize(self):
        blocks = {}
        i = iter(self.stack)
        try:
            nb = next(i)
        except StopIteration:
            return self._serialize(blocks, None, None)
        self._serialize(blocks, str(id(nb)), None)
        pb, cb = self, nb
        try:
            nb = next(i)
        except StopIteration:
            return cb._serialize(blocks, None, str(id(pb)))
        while True:
            cb._serialize(blocks, str(id(nb)), str(id(pb)))
            pb, cb = cb, nb
            try:
                nb = next(i)
            except StopIteration:
                return cb._serialize(blocks, None, str(id(pb)))


class gProcCall(gBlock):
    def __init__(self, name: str, args: dict[str, InputType]):
        self.name = name
        self.args = args.keys()
        self.inputs = args
        self.fields = {}

    def __repr__(self) -> str:
        return f"gProcCall(name={self.name!r}, args={self.inputs!r})"

    def __str__(self) -> str:
        return repr(self)

    def __rich_repr__(self):
        yield "name", self.name
        yield "args", self.inputs

    def _serialize(
        self,
        blocks: dict[str, Any],
        next_id: Optional[str],
        parent_id: Optional[str],
    ):
        blocks[str(id(self))] = {
            "opcode": "procedures_call",
            "next": next_id,
            "parent": parent_id,
            "inputs": self.serialize_inputs(blocks),
            "fields": self.serialize_fields(),
            "topLevel": False,
            "mutation": {
                "tagName": "mutation",
                "children": [],
                "proccode": self.name + " " + " ".join(["%s"] * len(self.args)),
                "argumentids": json.dumps(list(self.args)),
                "warp": True,
            },
        }
        for child in self.inputs.values():
            if isinstance(child, gBlock):
                child._serialize(blocks, None, str(id(self)))
        return blocks


class gProcProto(gBlock):
    def __init__(
        self,
        name: str,
        args: tuple[str, ...],
        warp: bool,
    ):
        self.name = name
        self.args = args
        self.warp = warp
        self.inputs = {
            i: gBlock("argument_reporter_string_number", {}, {"VALUE": i})
            for i in self.args
        }
        self.fields = {}

    def __repr__(self) -> str:
        return f"gProcProto(name={self.name!r}, args={self.args!r}, warp={self.warp!r})"

    def __str__(self) -> str:
        return repr(self)

    def __rich_repr__(self):
        yield "name", self.name
        yield "args", self.args
        yield "warp", self.warp

    def _serialize(
        self,
        blocks: dict[str, Any],
        next_id: Optional[str],
        parent_id: Optional[str],
    ):
        blocks[str(id(self))] = {
            "opcode": "procedures_prototype",
            "next": next_id,
            "parent": parent_id,
            "inputs": self.serialize_inputs(blocks),
            "fields": self.serialize_fields(),
            "topLevel": False,
            "shadow": True,
            "mutation": {
                "tagName": "mutation",
                "children": [],
                "proccode": self.name + " " + " ".join(["%s"] * len(self.args)),
                "argumentids": json.dumps(list(self.args)),
                "argumentnames": json.dumps(list(self.args)),
                "argumentdefaults": json.dumps([""] * len(self.args)),
                "warp": json.dumps(self.warp),
            },
        }
        for child in self.inputs.values():
            if isinstance(child, gBlock):
                child._serialize(blocks, None, None)
        return blocks


class gProcDef(gHatBlock):
    def __init__(
        self,
        name: str,
        args: tuple[str, ...],
        warp: bool,
        stack: tuple[gBlock, ...],
    ):
        self.opcode = "procedures_definition"
        self.name = name
        self.args = args
        self.warp = warp
        self.stack = stack
        self.inputs = {"custom_block": gProcProto(name, args, warp)}
        self.fields = {}

    def __repr__(self) -> str:
        return f"gProcDef(name={self.name!r}, args={self.args!r}, warp={self.warp!r}, stack={self.stack!r})"

    def __str__(self) -> str:
        return repr(self)

    def __rich_repr__(self):
        yield "name", self.name
        yield "args", self.args
        yield "warp", self.warp
        yield "stack", self.stack


class gArg(gBlock):
    def __init__(self, name: str):
        self.name = name
        self.opcode = "argument_reporter_string_number"
        self.inputs = {}
        self.fields = {"VALUE": name}

    def __repr__(self) -> str:
        return f"gArg({self.name!r})"

    def __str__(self) -> str:
        return repr(self)

    def __rich_repr__(self):
        yield "name", self.name
