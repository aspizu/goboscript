import sys
from pathlib import Path

from gbuild import build_gproject
from gerror import gError
from sb3.gblockfactory import reporter_prototypes, statement_prototypes


def print_table(table: list[str], width: int) -> None:
    maxwidth = 1 + max(map(len, table))
    columns = 1 + width // maxwidth
    table_iter = iter(table)
    while True:
        try:
            row: list[str] = []
            for _ in range(columns):
                row.append(next(table_iter))
            print("".join(i.ljust(maxwidth) for i in row))
        except StopIteration:
            break


def doc_statements() -> None:
    table: list[str] = []
    for prototype in statement_prototypes.values():
        table.append(
            prototype.name
            + (" " if prototype.arguments else "")
            + ", ".join(prototype.arguments)
            + ";"
        )
    print_table(table, width=80)


def doc_reporters() -> None:
    table: list[str] = []
    for prototype in reporter_prototypes.values():
        table.append(prototype.name + "(" + ", ".join(prototype.arguments) + ")")
    print_table(table, width=80)


def parse_args():
    if len(sys.argv) != 3:
        raise gError(
            "Expected 2 arguments",
            "gsc <project dir path> <output sb3 path>\nOr --doc statements/reporters to view documentation",
        )
    if sys.argv[1] == "--doc":
        if sys.argv[2] == "statements":
            doc_statements()
        elif sys.argv[2] == "reporters":
            doc_reporters()
        else:
            raise gError(
                f"Invalid argument {sys.argv[2]}", "gsc --doc statements/reporters"
            )
        return
    project = Path(sys.argv[1])
    output = Path(sys.argv[2])
    build_gproject(project).package(output)


def main():
    try:
        parse_args()
    except gError as e:
        e.print()
        exit(1)


if __name__ == "__main__":
    main()
