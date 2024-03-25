import json
from dataclasses import dataclass
from typing import Literal

from rich import print


@dataclass
class UnOp:
    opcode: str
    input: str
    fields: dict[str, str]


@dataclass
class BinOp:
    opcode: str
    lhs: str
    rhs: str


@dataclass
class Menu:
    input: str
    opcode: str
    default: str


@dataclass
class Block:
    name: str
    opcode: str
    args: list[str]
    fields: dict[str, str]
    menu: Menu | None


def snake_to_pascal(s: str):
    return "".join(x.title() for x in s.split("_"))


def table_split(table: str, n: int):
    xs = table.split()
    xs.extend([""] * (n - len(xs)))
    return xs


def parse():
    un_ops: dict[str, UnOp | None] = {}
    bin_ops: dict[str, BinOp | None] = {}
    blocks: dict[str, Block | list[Block]] = {}
    reporters: dict[str, Block | list[Block]] = {}
    old_opcode = ""
    old_input = ""
    old_lhs = ""
    old_rhs = ""
    old_fields: list[str] = []
    old_menu = ""
    opcode_prefix = ""
    old_args = ""
    section: Literal["UNARY", "BINARY", "BLOCKS", "REPORTERS"] | None = None
    lines = iter(open("gdsl.txt"))
    for line in lines:
        line = line[:-1].strip()
        if line.startswith("#") or not line:
            continue
        print(line)
        if line in ["UNARY OPERATORS", "BINARY OPERATORS", "BLOCKS", "REPORTERS"]:
            section = line.split()[0]  # type: ignore
            next(lines, None)
            next(lines, None)
            next(lines, None)
            continue
        if section == "UNARY":
            if line.endswith("~"):
                un_ops[line.removesuffix("~")] = None
                continue
            table, fields = line.split("|")
            fields = fields.strip()
            fields = (
                dict(
                    (old_fields[i] if key == "..." else key, value)
                    for i, (key, value) in enumerate(
                        x.split("=") for x in fields.strip().split(",")
                    )
                )
                if fields
                else {}
            )
            old_fields = list(fields.keys())
            variant, opcode, input = table.split()
            if opcode == "...":
                opcode = old_opcode
            else:
                opcode = "operator_" + opcode
                old_opcode = opcode
            if input == "...":
                input = old_input
            else:
                old_input = input
            un_ops[variant] = UnOp(opcode, input, fields)
        elif section == "BINARY":
            if line.endswith("~"):
                bin_ops[line.removesuffix("~")] = None
                continue
            variant, opcode, lhs, rhs = line.split()
            if opcode == "...":
                opcode = old_opcode
            else:
                opcode = "operator_" + opcode
                old_opcode = opcode
            if lhs == "...":
                lhs = old_lhs
            else:
                old_lhs = lhs
            if rhs == "...":
                rhs = old_rhs
            else:
                old_rhs = rhs
            bin_ops[variant] = BinOp(opcode, lhs, rhs)
        else:
            if line.startswith("["):
                opcode_prefix = line.split("]")[0].removeprefix("[")
                continue
            table, fields, menu = line.split("|")
            menu = menu.strip()
            if menu:
                input_opcode, default = menu.split("=")
                if input_opcode == "...":
                    input_opcode = old_menu
                old_menu = input_opcode
                input, opcode = input_opcode.split(":")
                menu = Menu(
                    input=input,
                    opcode=opcode,
                    default=default,
                )
            else:
                menu = None
            fields = fields.strip()
            fields = (
                dict(
                    (old_fields[i] if key == "..." else key, value)
                    for i, (key, value) in enumerate(
                        x.split("=") for x in fields.strip().split(",")
                    )
                )
                if fields
                else {}
            )
            old_fields = list(fields.keys())
            name, opcode, args = table_split(table, 3)
            variant = snake_to_pascal(name)
            if opcode == "...":
                opcode = old_opcode
            else:
                old_opcode = opcode
            opcode = f"{opcode_prefix}_{opcode}"
            if args == "...":
                args = old_args
            else:
                old_args = args
            args = args.split(",") if args else []
            if section == "BLOCKS":
                container = blocks
            else:
                container = reporters
            if variant in container:
                block = container[variant]
                if not isinstance(block, list):
                    block = [block]
                block.append(Block(name, opcode, args, fields, menu))
                container[variant] = block
            else:
                container[variant] = Block(name, opcode, args, fields, menu)
    return un_ops, bin_ops, blocks, reporters


un_ops, bin_ops, blocks, reporters = parse()

f = open("src/blocks.rs", "w")
f.write("""
pub struct Menu {
    pub input: &'static str,
    pub opcode: &'static str,
    pub default: &'static str,
}
""")
f.write("#[derive(Debug, Copy, Clone)]\npub enum UnOp {")
for un_op in un_ops:
    f.write(f"{un_op},")
f.write("}\n\n")
f.write("impl UnOp {")
f.write("pub fn opcode(&self) -> &'static str {")
f.write("match self {")
for variant, op in un_ops.items():
    if not op:
        continue
    f.write(f'Self::{variant} => "{op.opcode}",')
f.write("_ => unreachable!()")
f.write("}")
f.write("}\n\n")
f.write("pub fn input(&self) -> &'static str {")
f.write("match self {")
for variant, op in un_ops.items():
    if not op:
        continue
    f.write(f'Self::{variant} => "{op.input}",')
f.write("_ => unreachable!()")
f.write("}")
f.write("}\n\n")
f.write("pub fn fields(&self) -> Option<&'static str> {")
f.write("match self {")
for variant, op in un_ops.items():
    if not op:
        continue
    if len(op.fields) == 0:
        f.write(f"Self::{variant} => None,")
    else:
        f.write(
            f"Self::{variant} => Some({json.dumps(json.dumps({k:[v,None] for k,v in op.fields.items()}))}),"
        )
f.write("_ => unreachable!()")
f.write("}")
f.write("}")
f.write("}\n\n")
f.write("#[derive(Debug, Copy, Clone)]\npub enum BinOp {")
for bin_op in bin_ops:
    f.write(f"{bin_op},")
f.write("}\n\n")
f.write("impl BinOp {")
f.write("pub fn opcode(&self) -> &'static str {")
f.write("match self {")
for variant, op in bin_ops.items():
    if not op:
        continue
    f.write(f'Self::{variant} => "{op.opcode}",')
f.write("_ => unreachable!()")
f.write("}")
f.write("}\n\n")
f.write("pub fn lhs(&self) -> &'static str {")
f.write("match self {")
for variant, op in bin_ops.items():
    if not op:
        continue
    f.write(f'Self::{variant} => "{op.lhs}",')
f.write("_ => unreachable!()")
f.write("}")
f.write("}\n\n")
f.write("pub fn rhs(&self) -> &'static str {")
f.write("match self {")
for variant, op in bin_ops.items():
    if not op:
        continue
    f.write(f'Self::{variant} => "{op.rhs}",')
f.write("_ => unreachable!()")
f.write("}")
f.write("}")
f.write("}")


def write_blocks(typename: str, blocks: dict[str, Block | list[Block]]):
    f.write(f"#[derive(Debug, Copy, Clone)]\npub enum {typename} {{")
    for variant, block in blocks.items():
        if isinstance(block, list):
            for block in block:
                f.write(f"{variant}{len(block.args)},")
        else:
            f.write(f"{variant},")
    f.write("}\n\n")
    f.write(f"impl {typename} {{")
    f.write("pub fn menu(&self) -> Option<Menu> {")
    f.write("match self {")
    for variant, block in blocks.items():
        if isinstance(block, list):
            for block in block:
                avariant = f"{variant}{len(block.args)}"
                if block.menu:
                    f.write(
                        f"Self::{avariant} => Some(Menu {{ opcode: {json.dumps(block.menu.opcode)}, input: {json.dumps(block.menu.input)}, default: {json.dumps(block.menu.default)} }}),"
                    )
        elif block.menu:
            f.write(
                f"Self::{variant} => Some(Menu {{ opcode: {json.dumps(block.menu.opcode)}, input: {json.dumps(block.menu.input)}, default: {json.dumps(block.menu.default)} }}),"
            )
    f.write("_ => None }")
    f.write("}\n\n")
    f.write("pub fn overloads(name: &str) -> &'static [Self] {")
    f.write("match name {")
    for variant, block in blocks.items():
        if isinstance(block, list):
            variants = ",".join(f"Self::{variant}{len(b.args)}" for b in block)
            f.write(f'"{block[0].name}" => &[{variants}],')
    f.write("_ => &[] }")
    f.write("}\n\n")
    f.write("pub fn from_shape(name: &str, args: usize) -> Option<Self> {")
    f.write("match (name, args) {")
    for variant, block in blocks.items():
        if isinstance(block, list):
            b = block
            for block in block:
                f.write(
                    f'("{block.name}", {len(block.args)}) => Some(Self::{variant}{len(block.args)}),'
                )
            f.write(f'("{b[0].name}", _) => Some(Self::{variant}{len(b[0].args)}),')
        else:
            f.write(f'("{block.name}", _) => Some(Self::{variant}),')
    f.write("_ => None")
    f.write("}")
    f.write("}\n\n")
    f.write("pub fn name(&self) -> &'static str {")
    f.write("match self {")
    for variant, block in blocks.items():
        if not block:
            continue
        if isinstance(block, list):
            for block in block:
                f.write(
                    f"Self::{variant}{len(block.args)} => {json.dumps(block.name)},"
                )
        else:
            f.write(f"Self::{variant} => {json.dumps(block.name)},")
    f.write("}")
    f.write("}\n\n")
    f.write("pub fn all_names() -> &'static [&'static str] {")
    f.write("&[")
    for variant, block in blocks.items():
        if not block:
            continue
        if isinstance(block, list):
            block = block[0]
        f.write(f"{json.dumps(block.name)},")
    f.write("]")
    f.write("}\n\n")
    f.write("pub fn opcode(&self) -> &'static str {")
    f.write("match self {")
    for variant, block in blocks.items():
        if not block:
            continue
        if isinstance(block, list):
            for block in block:
                f.write(
                    f"Self::{variant}{len(block.args)} => {json.dumps(block.opcode)},"
                )
        else:
            f.write(f"Self::{variant} => {json.dumps(block.opcode)},")
    f.write("}")
    f.write("}\n\n")
    f.write("pub fn args(&self) -> &'static [&'static str] {")
    f.write("match self {")
    for variant, block in blocks.items():
        if not block:
            continue
        if isinstance(block, list):
            for block in block:
                f.write(
                    f"Self::{variant}{len(block.args)} => &{json.dumps(block.args)},"
                )
        else:
            f.write(f"Self::{variant} => &{json.dumps(block.args)},")
    f.write("}")
    f.write("}\n\n")
    f.write("pub fn fields(&self) -> Option<&'static str> {")
    f.write("match self {")
    for variant, block in blocks.items():
        if not block:
            continue
        if isinstance(block, list):
            for block in block:
                if len(block.fields) == 0:
                    f.write(f"Self::{variant}{len(block.args)} => None,")
                    continue
                f.write(
                    f"Self::{variant}{len(block.args)} => Some({json.dumps(json.dumps({k:[v,None] for k,v in block.fields.items()}))}),"
                )
        else:
            if len(block.fields) == 0:
                f.write(f"Self::{variant} => None,")
                continue
            f.write(
                f"Self::{variant} => Some({json.dumps(json.dumps({k:[v,None] for k,v in block.fields.items()}))}),"
            )
    f.write("}")
    f.write("}\n\n")
    f.write("}")


write_blocks("Block", blocks)
write_blocks("Repr", reporters)

print(
    json.dumps(
        f'\\b({"|".join(block.name if isinstance(block, Block) else block[0].name for block in blocks.values() )})\\b'
    )
)
print()
print(
    json.dumps(
        f'\\b({"|".join(block.name if isinstance(block, Block) else block[0].name for block in reporters.values() )})\\b'
    )
)
