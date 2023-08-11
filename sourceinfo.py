from typing import NamedTuple
from rich import print
from pathlib import Path


class Token(NamedTuple):
    value: str
    line: int  # 1-indexed line number
    column: int  # 1-indexed column number


class Function(NamedTuple):
    name: Token
    arguments: list[Token]
    doc: Token | None = None


class Macro(NamedTuple):
    name: Token
    arguments: list[Token]
    doc: Token | None = None


class BlockMacro(NamedTuple):
    name: Token
    arguments: list[Token]
    doc: Token | None = None


def tokenize(source: str) -> list[Token]:
    symbols = ",()"
    quote = False
    comment = False
    tokens: list[Token] = []
    line = 1
    column = 1
    start_line, start_column = line, column
    value = ""
    i = 0
    while i < len(source):
        if comment:
            if source[i] == "*" and source[i + 1] == "/":
                value += "*/"
                i += 1
                column += 1
                comment = False
                tokens.append(Token(value, start_line, start_column))
                value = ""
                start_line, start_column = line, column
            else:
                if source[i] == "\n":
                    line += 1
                    column = 1
                value += source[i]
        elif quote:
            if source[i] == "\\" and i + 1 < len(source) and source[i + 1] == '"':
                value += '\\"'
                i += 1
                column += 1
            elif source[i] == '"':
                quote = False
                value += '"'
                tokens.append(Token(value, start_line, start_column))
                value = ""
                start_line, start_column = line, column
            else:
                value += source[i]
        else:
            if source[i] == "/" and source[i + 1] == "*":
                comment = True
                value = "/*"
                column += 1
                tokens.append(Token(value, start_line, start_column))
                value = ""
                start_line, start_column = line, column
            elif source[i] in symbols:
                if value != "":
                    tokens.append(Token(value, start_line, start_column))
                tokens.append(Token(source[i], line, column))
                value = ""
                start_line, start_column = line, column
            elif source[i] in " \n":
                if value != "":
                    tokens.append(Token(value, start_line, start_column))
                value = ""
                start_line, start_column = line, column
                if source[i] == "\n":
                    line += 1
                    column = 1
                else:
                    column += 1
            elif source[i] == '"':
                quote = True
                value = '"'
                tokens.append(Token(value, start_line, start_column))
                value = ""
                start_line, start_column = line, column
            else:
                value += source[i]
        i += 1
        column += 1
    return tokens


path = Path("test/main.gs")
source = path.read_text()
tokens = tokenize(source)

stream = iter(tokens)

functions: list[Function] = []
macros: list[Macro] = []
block_macros: list[BlockMacro] = []
doc: Token | None = None
for token in stream:
    if token.value.startswith("/*"):
        doc = token
    elif token.value == "def":
        function: list[Token] = []
        while (current := next(stream)).value != "{":
            if current.value != ",":
                function.append(current)
        functions.append(Function(function[0], function[1:], doc))
        doc = None
    elif token.value == "macro":
        name = next(stream)
        args: list[Token] = []
        arg_or_bracket = next(stream)
        if arg_or_bracket.value == "(":
            while (current := next(stream)).value != "->":
                if current.value != ",":
                    args.append(current)
            macros.append(Macro(name, args, doc))
            doc = None
        else:
            args.append(arg_or_bracket)
            while (current := next(stream)).value != "{":
                if current.value != ",":
                    args.append(current)
            block_macros.append(BlockMacro(name, args, doc))
            doc = None
    else:
        doc = None

print("Functions:")
for i in functions:
    print(f"{i.name.value} {path}:{i.name.line}:{i.name.column}")

print("\nMacros:")
for i in macros:
    print(f"{i.name.value} {path}:{i.name.line}:{i.name.column}")

print("\nBlock Macros:")
for i in block_macros:
    print(f"{i.name.value} {path}:{i.name.line}:{i.name.column}")
