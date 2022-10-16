from dataclasses import dataclass


@dataclass(frozen=True)
class gVariable:
    name: str
    value: str
