from __future__ import annotations
from typing import TYPE_CHECKING
from hashlib import md5

if TYPE_CHECKING:
    from pathlib import Path
    from lib import JSON


def md5_hexdigest(path: Path):
    with path.open("rb") as f:
        file_hash = md5()
        while chunk := f.read(8192):
            file_hash.update(chunk)
    return file_hash.hexdigest()


class Costume:
    def __init__(self, path: Path, alias: str | None = None):
        self.path = path
        self.alias = alias
        self.hash = md5_hexdigest(self.path)
        self.md5ext = self.hash + self.path.suffix
        self.name = self.path.name.removesuffix(self.path.suffix).replace(
            "{{fwslash}}", "/"
        )

    def serialize(self) -> JSON:
        return {
            "name": self.name if self.alias is None else self.alias,
            "assetId": self.hash,
            "bitmapResolution": 1,
            "dataFormat": self.path.suffix[1:],
            "md5ext": self.md5ext,
        }
