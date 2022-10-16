from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class gSound:
    @staticmethod
    def from_path(path: Path) -> "gSound":
        raise NotImplementedError

    def serialize(self):
        raise NotImplementedError
