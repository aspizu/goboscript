import json
from typing import Any, Union

from lark.lexer import Token
from lib import JSON, tripletwise

from .gblockfactory import gPrototype

gInputType = Union[str, "gBlock", "gStack", "gVariable", "gList"]
gFieldType = Union[str, "gVariable", "gList"]
gBlockListType = dict[str, dict[str, JSON]]


def proccode(name: str, inputs: dict[str, "gArgument"]):
    args = [i.fields["VALUE"] for i in inputs.values()]
    return name + " " + " ".join([f"{arg}: %s" for arg in args])


class gVariable(str):
    ...


class gList:
    def __init__(self, name: str, data: list[str] | None = None):
        self.name = name
        self.data = data or []


class gBlock:
    def __init__(
        self,
        opcode: str,
        inputs: dict[str, gInputType],
        fields: dict[str, gFieldType],
        comment: str | None = None,
    ):
        self.opcode = opcode
        self.inputs = inputs
        self.fields = fields
        self.comment: str | None = comment
        self.x = 0
        self.y = 0
        self.id = str(id(self))

    @classmethod
    def from_prototype(
        cls,
        prototype: gPrototype,
        arguments: list[gInputType],
        comment: str | None = None,
    ):
        opcode = prototype.opcode
        fields: dict[str, gFieldType] = {}
        inputs: dict[str, gInputType] = {}
        if "." in prototype.opcode:
            opcode, fields = prototype.opcode.split(".")  # type: ignore
            fields = (i.split("=") for i in fields.split(","))  # type: ignore
            fields = {k: v for k, v in fields}
        elif "!" in prototype.opcode:
            opcode, inputs = prototype.opcode.split("!")  # type: ignore
            inputs = (i.split("=") for i in inputs.split(","))  # type: ignore
            inputs = {k: v for k, v in inputs}
        return cls(
            opcode,
            {**dict(zip(prototype.arguments, arguments)), **inputs},
            fields,
            comment,
        )

    def __rich_repr__(self) -> Any:
        yield "opcode", self.opcode
        yield "inputs", self.inputs
        yield "fields", self.fields

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.opcode}, {self.inputs}, {self.fields})"

    def serialize_input(
        self, blocks: gBlockListType, value: gInputType, name: str
    ) -> JSON:
        if type(value) is str:
            return [1, [10, value]]
        elif type(value) is gVariable:
            return [3, [12, value, value], [10, ""]]
        elif type(value) is gList:
            return [3, [13, value.name, ""], [10, ""]]
        elif isinstance(value, gStack):
            value.serialize(blocks, self.id)
            if len(value) == 0:
                return []
            else:
                return [2, value[0].id]
        elif isinstance(value, gBlock):
            value.serialize(blocks, None, self.id)
            if "CONDITION" in name:
                return [2, value.id]
            return [3, value.id, [10, ""]]
        raise ValueError(self, value)

    def serialize_field(self, blocks: gBlockListType, value: gFieldType) -> JSON:
        if isinstance(value, gVariable):
            return [value, value]
        if isinstance(value, gList):
            return [value.name, value.name]
        else:
            return [value, None]

    def serialize_inputs(self, blocks: gBlockListType):
        return {
            name: self.serialize_input(blocks, value, name)
            for name, value in self.inputs.items()
        }

    def serialize_fields(self, blocks: gBlockListType):
        return {
            name: self.serialize_field(blocks, value)
            for name, value in self.fields.items()
        }

    def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
        blocks[self.id] = {
            "opcode": self.opcode,
            "next": next,
            "parent": parent,
            "inputs": self.serialize_inputs(blocks),
            "fields": self.serialize_fields(blocks),
            "topLevel": isinstance(self, gHatBlock),
            "shadow": type(self) is gProcProto,
        }
        if blocks[self.id]["topLevel"]:
            blocks[self.id]["x"] = self.x
            blocks[self.id]["y"] = self.y
        if self.comment:
            blocks[self.id]["comment"] = self.comment


class gStack(list[gBlock]):
    def serialize(self, blocks: gBlockListType, parent: str):
        for prev, this, next in tripletwise(self):
            this.serialize(blocks, next and next.id, prev.id if prev else parent)


class gHatBlock(gBlock):
    def __init__(
        self,
        opcode: str,
        inputs: dict[str, gInputType],
        fields: dict[str, gFieldType],
        stack: gStack,
    ):
        super().__init__(opcode, inputs, fields)
        self.stack = stack

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.opcode}, {self.inputs}, {self.fields}, {self.stack})"

    def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
        super().serialize(
            blocks, self.stack[0].id if len(self.stack) > 0 else None, parent
        )
        self.stack.serialize(blocks, self.id)


class gArgument(gBlock):
    def __init__(self, name: str, shadow: bool = False):
        super().__init__("argument_reporter_string_number", {}, {"VALUE": name})
        self.shadow = shadow

    def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
        super().serialize(blocks, next, parent)
        if self.shadow:
            blocks[self.id]["shadow"] = True


class gProcCall(gBlock):
    def __init__(
        self, name: str, inputs: dict[str, gInputType], warp: bool, comment: str | None
    ):
        super().__init__("procedures_call", inputs, {}, comment)
        self.name = name
        self.warp = warp

    def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
        super().serialize(blocks, next, parent)
        blocks[self.id]["mutation"] = {
            "tagName": "mutation",
            "children": [],
            "proccode": proccode(
                self.name, {i: gArgument(i) for i in self.inputs.keys()}
            ),
            "argumentids": json.dumps(list(self.inputs.keys())),
            "warp": self.warp,
        }


class gProcProto(gBlock):
    def __init__(self, name: str, arguments: list[Token], warp: bool):
        super().__init__(
            "procedures_prototype",
            {argument: gArgument(argument, shadow=True) for argument in arguments},
            {},
        )
        self.name = name
        self.warp = warp

    def serialize(self, blocks: gBlockListType, next: str | None, parent: str | None):
        super().serialize(blocks, next, parent)
        argumentids = json.dumps(list(self.inputs.keys()))
        blocks[self.id]["mutation"] = {
            "tagName": "mutation",
            "children": [],
            "proccode": proccode(self.name, self.inputs),
            "argumentids": argumentids,
            "argumentnames": argumentids,
            "argumentdefaults": json.dumps(["0"] * len(self.inputs)),
            "warp": json.dumps(self.warp),
        }


class gProcDef(gHatBlock):
    def __init__(
        self,
        name: str,
        arguments: list[Token],
        warp: bool,
        stack: gStack,
    ):
        super().__init__(
            "procedures_definition",
            {"custom_block": gProcProto(name, arguments, warp)},
            {},
            stack,
        )
