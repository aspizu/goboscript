import pathlib


def data(path: pathlib.Path) -> list[str]:
    with open(path) as f:
        return f.read().splitlines(keepends=False)


def part1(path: pathlib.Path) -> int:
    d = data(path)
    print("width", len(d[0]))
    print("height", len(d))

    def helper(xmas: list[str]) -> int:
        return sum(
            ("".join(xmas[i][j : j + 4]) in ("XMAS", "SAMX"))
            + (
                i < len(xmas) - 3
                and "".join(
                    [
                        xmas[i][j],
                        xmas[i + 1][j + 1],
                        xmas[i + 2][j + 2],
                        xmas[i + 3][j + 3],
                    ]
                )
                in ("XMAS", "SAMX")
            )
            for i in range(len(xmas))
            for j in range(len(xmas[0]) - 3)
        )

    return sum(map(helper, [data(path), [line[::-1] for line in zip(*data(path))]]))


path = pathlib.Path(__file__).parent.joinpath("input.txt")
print(f"Part 1: {part1(path)}")
