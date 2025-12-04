def solve(lines: list[str], need_on: int):
    result = 0
    for line in lines:
        res = max_banks(line, need_on)

        result += res

    return result


def max_banks(line: str, need_on: int) -> int:
    digits = list(map(int, line))

    res = 0
    search_pos = 0

    context_window = len(digits) - need_on

    for i in range(need_on):
        current_view = digits[search_pos : search_pos + context_window + 1]
        max_cur = max(current_view)
        inside_pos_cur = current_view.index(max_cur)
        context_window -= inside_pos_cur
        search_pos += inside_pos_cur + 1
        res *= 10
        res += max_cur

    return res


def part_1(lines: list[str]) -> int | str:
    return solve(lines, 2)


def part_2(lines: list[str]) -> int | str:
    return solve(lines, 12)
