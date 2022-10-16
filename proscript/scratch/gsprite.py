from dataclasses import dataclass

from .gblock import gBlock
from .gcostume import gCostume
from .glist import gList
from .gsound import gSound
from .gvariable import gVariable


@dataclass(frozen=True)
class gSprite:
    name: str
    variables: tuple[gVariable, ...]
    lists: tuple[gList, ...]
    blocks: tuple[gBlock, ...]
    costumes: tuple[gCostume, ...]
    sounds: tuple[gSound, ...]

    def serialize(self):
        return {
            "isStage": self.name == "Stage",
            "name": self.name,
            "variables": {i.name: [i.name, i.value] for i in self.variables},
            "lists": {i.name: [i.name, i.values] for i in self.lists},
            "blocks": {
                k: v for d in (i.serialize() for i in self.blocks) for k, v in d.items()
            },
            "costumes": [i.serialize() for i in self.costumes],
            "sounds": [i.serialize() for i in self.sounds],
        }
