import json
from io import TextIOWrapper
from itertools import chain
from pathlib import Path
from zipfile import ZipFile

from lib import JSON

from .gsprite import gSprite


class gProject:
    def __init__(self, stage: gSprite, sprites: list[gSprite]):
        self.stage = stage
        self.sprites = sprites

    def serialize(self) -> JSON:
        return {
            "targets": [
                sprite.serialize() for sprite in chain([self.stage], self.sprites)
            ],
            "meta": {"semver": "3.0.0"},
        }

    def package(self, path: Path):
        with ZipFile(path, "w") as file:
            for costume in chain(*(sprite.costumes for sprite in self.sprites)):
                file.write(costume.path, costume.md5ext)
            with file.open("project.json", "w") as project_json:
                json.dump(self.serialize(), TextIOWrapper(project_json))
