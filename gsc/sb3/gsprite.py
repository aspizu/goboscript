from typing import cast

from lib import JSON

from .gblock import gBlock, gBlockListType, gList, gVariable
from .gcostume import gCostume


class gSprite:
    def __init__(
        self,
        name: str,
        variables: dict[str, gVariable],
        lists: dict[str, gList],
        blocks: list[gBlock],
        costumes: list[gCostume],
        comment: str | None = None,
    ):
        self.name = name
        self.variables = variables
        self.lists = lists
        self.blocks = blocks
        self.costumes = costumes
        self.comment = comment

    def serialize(self) -> dict[str, JSON]:
        assert len(self.costumes) > 0
        blocks: gBlockListType = {}
        for block in self.blocks:
            block.serialize(blocks, None, None)
        comments: dict[str, dict[str, JSON]] = {}
        for id, block in blocks.items():
            if "comment" in block:
                assert isinstance(block["comment"], str)
                comments[id + block["comment"]] = {
                    "blockId": id,
                    "minimized": False,
                    "text": block["comment"],
                }
        if self.comment:
            comments["__docComment__"] = {
                "blockId": None,
                "minimized": False,
                "text": self.comment,
                "width": 200,
                "height": 200,
                "x": 0,
                "y": 0,
            }
        return {
            "isStage": self.name == "Stage",
            "name": self.name,
            "variables": {qualname: [qualname, 0] for qualname in self.variables},
            "lists": cast(
                JSON,
                {
                    qualname: [qualname, list_.data]
                    for qualname, list_ in self.lists.items()
                },
            ),
            "blocks": cast(JSON, blocks),
            "costumes": [costume.serialize() for costume in self.costumes],
            "comments": cast(JSON, comments),
            "sounds": [],
        }
