import json
from io import TextIOWrapper
from itertools import chain
from pathlib import Path
from typing import Any
from zipfile import ZipFile

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

    def package(self, path: Path):
        with ZipFile(path, "w") as zf:
            for sprite in self.sprites:
                for costume in sprite.costumes:
                    zf.write(costume.path, costume.name)
            with zf.open("project.json", mode="w") as fp:
                json.dump(self.serialize(), TextIOWrapper(fp))
