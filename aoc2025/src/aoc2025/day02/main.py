import itertools
from collections.abc import Callable


def solve(lines: list[str], is_invalid_id: Callable[[str], bool]):
    line = lines[0]

    result = 0

    for part in line.split(","):
        start, end = part.split("-")

        for i in range(int(start), int(end) + 1):
            invalid = is_invalid_id(str(i))
            if invalid:
                result += i

    return result


def part_1(lines: list[str]) -> int | str:
    return solve(lines, is_invalid_id_1)


def part_2(lines: list[str]) -> int | str:
    return solve(lines, is_invalid_id_2)


def is_invalid_id_1(id: str) -> bool:
    half_len = len(id) // 2
    return id[half_len:] in id[:half_len]


def is_invalid_id_2(id: str) -> bool:
    for i in range(1, len(id) // 2 + 1):
        groups_of_len = itertools.batched(id, i)
        if len(set(groups_of_len)) == 1:
            return True

    return False
