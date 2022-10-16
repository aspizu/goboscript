import io
import json
from dataclasses import dataclass
from pathlib import Path
from zipfile import ZipFile

from .gsprite import gSprite


@dataclass(frozen=True)
class gProject:
    stage: gSprite
    sprites: tuple[gSprite, ...]

    def serialize(self):
        return {
            "targets": [i.serialize() for i in (self.stage,) + self.sprites],
            "meta": {"semver": "3.0.0"},
        }

    def package(self, output_pth: Path) -> "gProject":
        with ZipFile(output_pth, mode="w") as sb3:
            for sprite in self.sprites:
                for costume in sprite.costumes:
                    sb3.write(costume.path, costume.hash + costume.path.suffix)
            with sb3.open("project.json", "w") as fp:
                json.dump(self.serialize(), io.TextIOWrapper(fp))
        return self
