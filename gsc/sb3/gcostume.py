from hashlib import md5
from pathlib import Path
from typing import Any


def md5_hexdigest(path: Path) -> str:
    with path.open("rb") as f:
        file_hash = md5()
        while chunk := f.read(8192):
            file_hash.update(chunk)
    return file_hash.hexdigest()


class gCostume:
    def __init__(self, path: Path) -> None:
        self.path = path
        self.hash = md5_hexdigest(self.path)
        self.name = self.hash + self.path.suffix

    def serialize(self) -> dict[str, Any]:
        return {
            "name": self.path.name,
            "assetId": self.hash,
            "dataFormat": self.path.suffix[1:],
            "md5ext": self.name,
        }

    def __rich_repr__(self):
        yield self.path
