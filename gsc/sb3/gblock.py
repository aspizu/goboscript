import json
from typing import Any, Optional, TypeAlias, Union

from .glist import gList
from .gvariable import gVariable

InputType: TypeAlias = Union[str, "gBlock", gVariable, gList, "gStack"]
FieldType: TypeAlias = Union[str, gVariable, gList]
SerializedBlockList: TypeAlias = dict[str, dict[str, Any]]


class gBlock:
    def __init__(
        self,
        opcode: str,
        inputs: dict[str, InputType],
        fields: dict[str, FieldType],
    ) -> None:
        self.opcode, self.inputs, self.fields = opcode, inputs, fields

    def serialize_input(self, o: InputType, blocks: SerializedBlockList):
        if isinstance(o, gStack):
            o.serialize(blocks)
            return [2, str(id(o[0]))]
        elif isinstance(o, gBlock):
            o.serialize(blocks, parent_id=str(id(self)))
            return [2, str(id(o))]
        elif isinstance(o, gVariable):
            return [2, [12, o.name, o.name]]
        elif isinstance(o, str):
            return [1, [10, o]]
        else:
            raise TypeError(type(o), o)

    def serialize_field(self, o: InputType, blocks: SerializedBlockList):
        ...

    def serialize(
        self,
        blocks: SerializedBlockList,
        next_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        toplevel: bool = False,
    ) -> None:
        blocks[str(id(self))] = {
            "opcode": self.opcode,
            "next": next_id,
            "parent": parent_id,
            "inputs": {
                k: self.serialize_input(v, blocks) for k, v in self.inputs.items()
            },
            "fields": {
                k: self.serialize_field(v, blocks) for k, v in self.fields.items()
            },
            "topLevel": toplevel,
        }

    def __rich_repr__(self):
        yield self.opcode
        yield self.inputs
        yield self.fields


class gStack(list[gBlock]):
    def serialize(
        self,
        blocks: SerializedBlockList,
        parent_id: Optional[str] = None,
    ) -> None:
        if len(self) > 0:
            self[0].serialize(blocks, str(id(self[1])), parent_id)
        if len(self) > 2:
            for i, o in enumerate(self[1:-1], 1):
                o.serialize(blocks, str(id(self[i + 1])), str(id(self[i - 1])))
        if len(self) > 1:
            self[-1].serialize(blocks, parent_id=str(id(self[-2])))


class gHatBlock(gBlock):
    def __init__(
        self,
        opcode: str,
        inputs: dict[str, InputType],
        fields: dict[str, FieldType],
        stack: gStack,
    ) -> None:
        self.opcode = opcode
        self.inputs = inputs
        self.fields = fields
        self.stack = stack

    def serialize(self, blocks: SerializedBlockList) -> None:
        super().serialize(blocks, toplevel=True)
        self.stack.serialize(blocks, str(id(self)))

    def __rich_repr__(self):
        yield from super().__rich_repr__()
        yield self.stack


class gProcCall(gBlock):
    def __init__(self, name: str, args: dict[str, InputType]):
        self.name, self.inputs, self.fields = name, args, {}

    def serialize(
        self,
        blocks: SerializedBlockList,
        next_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        toplevel: bool = False,
    ) -> None:
        blocks[str(id(self))] = {
            "opcode": "procedures_call",
            "next": next_id,
            "parent": parent_id,
            "inputs": {
                k: self.serialize_input(v, blocks) for k, v in self.inputs.items()
            },
            "fields": {
                k: self.serialize_field(v, blocks) for k, v in self.fields.items()
            },
            "topLevel": False,
            "mutation": {
                "tagName": "mutation",
                "children": [],
                "proccode": self.name + " " + " ".join(["%s"] * len(self.inputs)),
                "argumentids": json.dumps(self.inputs.keys()),
                "warp": True,  # FIXME
            },
        }

    def __rich_repr__(self):
        yield self.name
        yield self.inputs
        yield self.fields


class gProcProto(gBlock):
    def __init__(self, name: str, args: list[str], warp: bool = False):
        self.name, self.warp, self.fields = name, warp, {}
        self.inputs = {
            i: gBlock("argument_reporter_string_number", {}, {"VALUE": i}) for i in args
        }

    def serialize(
        self,
        blocks: SerializedBlockList,
        next_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        toplevel: bool = False,
    ) -> None:
        blocks[str(id(self))] = {
            "opcode": "procedures_prototype",
            "next": next_id,
            "parent": parent_id,
            "inputs": {
                k: self.serialize_input(v, blocks) for k, v in self.inputs.items()
            },
            "fields": {
                k: self.serialize_field(v, blocks) for k, v in self.fields.items()
            },
            "topLevel": toplevel,
            "shadow": True,
            "mutation": {
                "tagName": "mutation",
                "children": [],
                "proccode": self.name + " " + " ".join(["%s"] * len(self.inputs)),
                "argumentids": json.dumps(list(self.inputs.values())),
                "argumentnames": json.dumps(list(self.inputs.values())),
                "argumentdefaults": json.dumps(["0"] * len(self.inputs)),
                "warp": json.dumps(self.warp),
            },
        }

    def __rich_repr__(self):
        yield from (
            self.name,
            self.warp,
            self.inputs,
            self.fields,
        )


def new_proc_def(name: str, args: list[str], warp: bool, stack: gStack):
    return gHatBlock(
        "procedures_definition",
        {"custom_block": gProcProto(name, args, warp)},
        {},
        stack,
    )


def new_arg_reporter(name: str):
    return gBlock("argument_reporter_string_number", {}, {"VALUE": name})
