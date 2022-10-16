import hashlib
from dataclasses import dataclass
from pathlib import Path


def md5_hexdigest(path: Path) -> str:
    with path.open("rb") as f:
        file_hash = hashlib.md5()
        while chunk := f.read(8192):
            file_hash.update(chunk)
    return file_hash.hexdigest()


@dataclass(frozen=True)
class gCostume:
    name: str
    hash: str
    path: Path

    @staticmethod
    def from_path(path: Path) -> "gCostume":
        return gCostume(path.name, md5_hexdigest(path), path)

    def serialize(self):
        return {
            "name": self.path.name,
            "assetId": self.hash,
            "dataFormat": self.path.suffix[1:],
            "md5ext": self.hash + self.path.suffix,
        }
