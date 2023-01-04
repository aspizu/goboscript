from typing import cast

from lib import JSON

from .gblock import gBlock, gBlockListType, gList, gVariable
from .gcostume import gCostume


class gSprite:
    def __init__(
        self,
        name: str,
        variables: list[gVariable],
        lists: list[gList],
        blocks: list[gBlock],
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
        comments: dict[str, dict[str, JSON]] = {}
        for id, block in blocks.items():
            if block["comment"]:
                assert isinstance(block["comment"], str)
                comments[block["comment"]] = {
                    "blockId": id,
                    "minimized": True,
                    "text": block["comment"],
                }
        return {
            "isStage": self.name == "Stage",
            "name": self.name,
            "variables": {variable: [variable, 0] for variable in self.variables},
            "lists": {lst: [lst, []] for lst in self.lists},
            "blocks": cast(JSON, blocks),
            "costumes": [costume.serialize() for costume in self.costumes],
            "comments": cast(JSON, comments),
            "sounds": [],
        }
