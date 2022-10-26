from typing import Any

from .gblock import SerializedBlockList, gHatBlock
from .gcostume import gCostume
from .glist import gList
from .gvariable import gVariable


class gSprite:
    def __init__(
        self,
        name: str,
        variables: list[gVariable],
        lists: list[gList],
        blocks: list[gHatBlock],
        costumes: list[gCostume],
    ) -> None:
        self.name = name
        self.variables = variables
        self.lists = lists
        self.blocks = blocks
        self.costumes = costumes

    def serialize(self) -> dict[str, Any]:
        blocks: SerializedBlockList = {}
        for i in self.blocks:
            i.serialize(blocks)
        return {
            "isStage": self.name == "Stage",
            "name": self.name,
            "variables": {i.name: [i.name, i.value] for i in self.variables},
            "lists": {i.name: [i.name, i.values] for i in self.lists},
            "blocks": blocks,
            "costumes": [i.serialize() for i in self.costumes],
            "sounds": [],
        }

    def __rich_repr__(self):
        yield self.name
        yield self.variables
        yield self.lists
        yield self.blocks
        yield self.costumes
