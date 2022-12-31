from typing import cast

from lib import JSON

from .gblock import gBlockListType, gHatBlock, gList, gVariable
from .gcostume import gCostume


class gSprite:
    def __init__(
        self,
        name: str,
        variables: list[gVariable],
        lists: list[gList],
        blocks: list[gHatBlock],
        costumes: list[gCostume],
    ):
        self.name = name
        self.variables = variables
        self.lists = lists
        self.blocks = blocks
        self.costumes = costumes

    def serialize(self) -> dict[str, JSON]:
        assert len(self.costumes) > 0
        blocks: gBlockListType = {}
        for block in self.blocks:
            block.serialize(blocks, None, None)
        return {
            "isStage": self.name == "Stage",
            "name": self.name,
            "variables": {variable: [variable, 0] for variable in self.variables},
            "blocks": cast(JSON, blocks),
            "lists": {lst: [lst, []] for lst in self.lists},
            "costumes": [costume.serialize() for costume in self.costumes],
            "sounds": [],
        }
