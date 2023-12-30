from __future__ import annotations
import json
from typing import (
    TYPE_CHECKING,
    Any,
    Union,
    Mapping,
    Iterable,
    Sequence,
    NamedTuple,
    cast,
)
from ..lib import JSON, tripletwise

if TYPE_CHECKING:
    from lark.lexer import Token
    from .blockfactory import Prototype

Input = Union[str, "Block", "Stack", "Variable", "List"]
Field = Union[str, "Variable", "List"]
BlockListType = dict[str, dict[str, JSON]]


def proccode(name: str, inputs_: Mapping[str, Input]):
    inputs = cast(dict[str, Argument], inputs_)
    args = (i.fields["VALUE"] for i in inputs.values())
    return name + " 💀 " + " ".join(f"{arg}: %s" for arg in args)


class Variable(NamedTuple):
    name: str
    token: Token
    is_cloud: bool = False


class List:
    def __init__(self, token: Token, data: list[str] | None = None):
        self.token = token
        self.data = data or []


class Block:
    def __init__(
        self,
        opcode: str,
        inputs: Mapping[str, Input],
        fields: Mapping[str, Field],
        token: Token | None = None,
        comment: str | None = None,
    ):
        self.opcode = opcode
        self.inputs = inputs
        self.fields = fields
        self.comment: str | None = comment
        self.token = token
        self.x = 0
        self.y = 0
        self.id = str(id(self))

    @classmethod
    def from_prototype(
        cls,
        prototype: Prototype,
        arguments: Iterable[Input],
        token: Token | None = None,
        comment: str | None = None,
    ):
        opcode = prototype.opcode
        fields: Mapping[str, Field] = {}
        inputs: Mapping[str, Input] = {}
        if "." in prototype.opcode:
            opcode, fieldnamess = prototype.opcode.split(".")
            fields = dict(i.split("=") for i in fieldnamess.split(","))
        elif "!" in prototype.opcode:
            opcode, inputnames = prototype.opcode.split("!")
            inputs = dict(i.split("=") for i in inputnames.split(","))
        if prototype.name[-1] == "?":
            cls = ConditionBlock
        return cls(
            opcode=opcode,
            inputs={**dict(zip(prototype.arguments, arguments)), **inputs},
            fields=fields,
            token=token,
            comment=comment,
        )

    def __rich_repr__(self) -> Any:
        yield "opcode", self.opcode
        yield "inputs", self.inputs
        yield "fields", self.fields

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.opcode}, {self.inputs}, {self.fields})"

    def serialize_input(self, blocks: BlockListType, value: Input, name: str) -> JSON:
        if type(value) is Variable:
            return [3, [12, value.name, value.name], [10, ""]]
        if type(value) is List:
            return [3, [13, value.token, ""], [10, ""]]
        if isinstance(value, Stack):
            value.serialize(blocks, self.id)
            if len(value) == 0:
                return []
            return [2, value[0].id]
        if isinstance(value, Block):
            value.serialize(blocks, None, self.id)
            if isinstance(value, ConditionBlock):
                return [2, value.id]
            if (
                value.opcode == "sensing_of_object_menu"
                or name == "custom_block"
                or (isinstance(value, Argument) and value.shadow)
            ):
                return [1, value.id]
            return [3, value.id, [10, ""]]
        if type(value) is str:
            return [1, [10, value]]
        raise ValueError(self, value)

    def serialize_field(self, blocks: BlockListType, value: Field) -> JSON:
        if isinstance(value, Variable):
            return [value.name, value.name]
        if isinstance(value, List):
            return [value.token, value.token]
        return [value, None]

    def serialize_inputs(self, blocks: BlockListType):
        return {
            name: self.serialize_input(blocks, value, name)
            for name, value in self.inputs.items()
        }

    def serialize_fields(self, blocks: BlockListType):
        return {
            name: self.serialize_field(blocks, value)
            for name, value in self.fields.items()
        }

    def serialize(self, blocks: BlockListType, next: str | None, parent: str | None):
        blocks[self.id] = {
            "opcode": self.opcode,
            "next": next,
            "parent": parent,
            "inputs": self.serialize_inputs(blocks),
            "fields": self.serialize_fields(blocks),
            "topLevel": isinstance(self, HatBlock),
            "shadow": self.opcode == "sensing_of_object_menu",
        }
        if blocks[self.id]["topLevel"]:
            blocks[self.id]["x"] = self.x
            blocks[self.id]["y"] = self.y
        if self.comment:
            blocks[self.id]["comment"] = self.comment


class ConditionBlock(Block):
    ...


class Stack(list[Block]):
    @classmethod
    def new(cls, stack: Sequence[Block | Sequence[Block]]):
        new = cls()
        for i in stack:
            if isinstance(i, Sequence):
                new.extend(i)
            else:
                new.append(i)
        return new

    def serialize(self, blocks: BlockListType, parent: str):
        for prev, this, next in tripletwise(self):
            this.serialize(blocks, next and next.id, prev.id if prev else parent)


class HatBlock(Block):
    def __init__(
        self,
        opcode: str,
        inputs: dict[str, Input],
        fields: dict[str, Field],
        stack: Stack,
        token: Token | None = None,
    ):
        super().__init__(opcode, inputs, fields, token)
        self.stack = stack

    def serialize(self, blocks: BlockListType, next: str | None, parent: str | None):
        super().serialize(
            blocks, self.stack[0].id if len(self.stack) > 0 else None, parent
        )
        self.stack.serialize(blocks, self.id)


class Argument(Block):
    def __init__(self, name: str, token: Token | None = None, *, shadow: bool = False):
        super().__init__("argument_reporter_string_number", {}, {"VALUE": name}, token)
        self.shadow = shadow

    def serialize(self, blocks: BlockListType, next: str | None, parent: str | None):
        super().serialize(blocks, next, parent)
        if self.shadow:
            blocks[self.id]["shadow"] = True


class ProcCall(Block):
    def __init__(
        self,
        name: str,
        inputs: dict[str, Input],
        comment: str | None,
        proccode: str | None,
        token: Token | None = None,
        *,
        warp: bool,
    ):
        super().__init__("procedures_call", inputs, {}, token, comment)
        self.name = name
        self.warp = warp
        self.proccode = proccode

    def serialize(self, blocks: BlockListType, next: str | None, parent: str | None):
        super().serialize(blocks, next, parent)
        blocks[self.id]["mutation"] = {
            "tagName": "mutation",
            "children": [],
            "proccode": self.proccode
            or proccode(self.name, {i: Argument(i, None) for i in self.inputs}),
            "argumentids": json.dumps(list(self.inputs.keys())),
            "warp": self.warp,
        }


class ProcProto(Block):
    def __init__(
        self,
        name: str,
        arguments: list[Token],
        token: Token | None = None,
        *,
        warp: bool,
    ):
        super().__init__(
            "procedures_prototype",
            {argument: Argument(argument, None, shadow=True) for argument in arguments},
            {},
            token,
        )
        self.name = name
        self.warp = warp

    def serialize(self, blocks: BlockListType, next: str | None, parent: str | None):
        super().serialize(blocks, next, parent)
        argumentids = json.dumps(list(self.inputs.keys()))
        serialized = blocks[self.id]
        serialized["shadow"] = True
        serialized["mutation"] = {
            "tagName": "mutation",
            "children": [],
            "proccode": proccode(self.name, self.inputs),
            "argumentids": argumentids,
            "argumentnames": argumentids,
            "argumentdefaults": json.dumps(["0"] * len(self.inputs)),
            "warp": json.dumps(self.warp),
        }


class ProcDef(HatBlock):
    def __init__(
        self,
        name: str,
        arguments: list[Token],
        stack: Stack,
        token: Token | None = None,
        *,
        warp: bool,
    ):
        super().__init__(
            "procedures_definition",
            {"custom_block": ProcProto(name, arguments, token, warp=warp)},
            {},
            stack,
            token,
        )
