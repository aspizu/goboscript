from __future__ import annotations
from .block import Block, Input, HatBlock

PADDING = 30
HAT_BLOCK_HEIGHT = 65
BLOCK_SIZE = 48


def input_height(input: Input) -> int:
    if isinstance(input, Block):
        return 8 + max((*(input_height(i) for i in input.inputs.values()), 0))
    return 0


def height(block: Block) -> int:
    if isinstance(block, HatBlock):
        return HAT_BLOCK_HEIGHT + sum(height(i) for i in block.stack)
    return BLOCK_SIZE + max((*(input_height(i) for i in block.inputs.values()), 0))


def cleanup(blocks: list[Block]) -> None:
    y = 0
    for block in blocks:
        if isinstance(block, HatBlock):
            block.y = y
            y += height(block) + PADDING
