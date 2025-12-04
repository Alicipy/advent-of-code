import pytest

from aoc2025.day03.main import max_banks, part_1, part_2

TEST_INPUT = [
    "987654321111111",
    "811111111111119",
    "234234234234278",
    "818181911112111",
]

TEST_OUTPUT_1 = [98, 89, 78, 92]

TEST_OUTPUT_2 = [
    987654321111,
    811111111119,
    434234234278,
    888911112111,
]


def test__part_1__example():
    assert part_1(TEST_INPUT) == 357


def test__part_2__example():
    assert part_2(TEST_INPUT) == 3121910778619


@pytest.mark.parametrize("inp,output", list(zip(TEST_INPUT, TEST_OUTPUT_1)))
def test__max_banks_example_2_on(inp, output):
    assert max_banks(inp, 2) == output


@pytest.mark.parametrize("inp,output", list(zip(TEST_INPUT, TEST_OUTPUT_2)))
def test__max_banks_example_12_on(inp, output):
    assert max_banks(inp, 12) == output
