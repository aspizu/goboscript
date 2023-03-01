from hashlib import md5
from pathlib import Path

from lib import JSON


def md5_hexdigest(path: Path):
    with path.open("rb") as f:
        file_hash = md5()
        while chunk := f.read(8192):
            file_hash.update(chunk)
    return file_hash.hexdigest()


class gCostume:
    def __init__(self, path: Path):
        self.path = path
        self.hash = md5_hexdigest(self.path)
        self.md5ext = self.hash + self.path.suffix
        self.name = self.path.name.removesuffix(self.path.suffix).replace(
            "{{fwslash}}", "/"
        )

    def serialize(self) -> JSON:
        return {
            "name": self.name,
            "assetId": self.hash,
            "dataFormat": self.path.suffix[1:],
            "md5ext": self.md5ext,
        }
