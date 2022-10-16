from dataclasses import dataclass


@dataclass(frozen=True)
class gList:
    name: str
    values: tuple[str, ...]
