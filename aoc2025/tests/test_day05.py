from aoc2025.day05.main import part_1, part_2

TEST_INPUT = [
    "3-5",
    "10-14",
    "16-20",
    "12-18",
    "",
    "1",
    "5",
    "8",
    "11",
    "17",
    "32",
]


def test__part_1__example():
    assert part_1(TEST_INPUT) == 3


def test__part_2__example():
    assert part_2(TEST_INPUT) == 14


def test__part_2__cornercase_end():
    assert part_2(["2-3", "1-2", "4-5"]) == 5
