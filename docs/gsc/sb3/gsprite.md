# gSprite

[Goboscript Index](../../README.md#goboscript-index) /
`gsc` /
[Sb3](./index.md#sb3) /
gSprite

> Auto-generated documentation for [gsc.sb3.gsprite](../../../gsc/sb3/gsprite.py) module.

- [gSprite](#gsprite)
  - [gSprite](#gsprite-1)
    - [gSprite().serialize](#gsprite()serialize)

## gSprite

[Show source in gsprite.py:9](../../../gsc/sb3/gsprite.py#L9)

#### Signature

```python
class gSprite:
    def __init__(
        self,
        name: str,
        variables: list[gVariable],
        lists: list[gList],
        blocks: list[gBlock],
        costumes: list[gCostume],
        comment: str | None = None,
    ):
        ...
```

### gSprite().serialize

[Show source in gsprite.py:26](../../../gsc/sb3/gsprite.py#L26)

#### Signature

```python
def serialize(self) -> dict[str, JSON]:
    ...
```