from .gblock import gBlock, gHatBlock, gInputType

PADDING = 30
HAT_BLOCK_HEIGHT = 65
BLOCK_SIZE = 48


def input_height(input: gInputType) -> int:
    if isinstance(input, gBlock):
        return 8 + max(input_height(i) for i in input.inputs.values())
    return 0


def height(block: gBlock) -> int:
    if isinstance(block, gHatBlock):
        return HAT_BLOCK_HEIGHT + sum(height(i) for i in block.stack)
    return BLOCK_SIZE + max(input_height(i) for i in block.inputs.values())


def cleanup(blocks: list[gBlock]) -> None:
    y = 0
    for block in blocks:
        if isinstance(block, gHatBlock):
            block.y = y
            y += height(block) + PADDING
