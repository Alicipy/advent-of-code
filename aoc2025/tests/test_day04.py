from aoc2025.day04.main import part_1, part_2

TEST_INPUT = [
    "..@@.@@@@.",
    "@@@.@.@.@@",
    "@@@@@.@.@@",
    "@.@@@@..@.",
    "@@.@@@@.@@",
    ".@@@@@@@.@",
    ".@.@.@.@@@",
    "@.@@@.@@@@",
    ".@@@@@@@@.",
    "@.@.@@@.@.",
]


def test__part_1__example():
    assert part_1(TEST_INPUT) == 13


def test__part_2__example():
    assert part_2(TEST_INPUT) == 43
