from aoc2025.day01.main import part_1, part_2

TEST_INPUT = [
    "L68",
    "L30",
    "R48",
    "L5",
    "R60",
    "L55",
    "L1",
    "L99",
    "R14",
    "L82",
]


def test__part_1__example():
    assert part_1(TEST_INPUT) == 3


def test__part_2__example():
    assert part_2(TEST_INPUT) == 6
