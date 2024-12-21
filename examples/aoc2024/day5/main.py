from pathlib import Path

data = (
    Path(__file__).parent.joinpath("input.txt").read_text().splitlines(keepends=False)
)

rules: list[tuple[int, int]] = [
    tuple(map(int, line.split("|"))) for line in data if "|" in line
]

pages: list[list[int]] = [
    list(map(int, line.split(","))) for line in data if "," in line
]


def rule_in_page(rule: tuple[int, int], page: list[int]) -> bool:
    left, right = rule
    if left in page and right in page:
        left_idx = page.index(left)
        right_idx = page.index(right)
        return left_idx < right_idx
    else:
        return True


def rules_in_page(rules: list[tuple[int, int]], page: list[int]) -> bool:
    return all(rule_in_page(rule, page) for rule in rules)


def middle_number(page: list[int]) -> int:
    return page[len(page) // 2]


result = sum(middle_number(page) for page in pages if rules_in_page(rules, page))
print("Result: ", result)

result = 0
for page in pages:
    if not rules_in_page(rules, page):
        while not rules_in_page(rules, page):
            for left, right in rules:
                if left in page and right in page:
                    left_idx = page.index(left)
                    right_idx = page.index(right)
                    if left_idx > right_idx:
                        page[left_idx], page[right_idx] = (
                            page[right_idx],
                            page[left_idx],
                        )
                        break
        result += middle_number(page)

print("Result: ", result)
