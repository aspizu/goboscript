from itertools import chain
from typing import Any

from .gsprite import gSprite


class gProject:
    def __init__(self, stage: gSprite, sprites: list[gSprite]) -> None:
        self.stage = stage
        self.sprites = sprites

    def serialize(self) -> dict[str, Any]:
        return {
            "targets": [i.serialize() for i in chain([self.stage], self.sprites)],
            "meta": {"semver": "3.0.0"},
        }
