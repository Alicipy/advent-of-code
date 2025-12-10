from aoc2025.day09.main import part_1, part_2

TEST_INPUT = """
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
""".strip().splitlines()


def test__part_1__solve_example():
    assert part_1(TEST_INPUT) == 50


def test__part_2__solve_example():
    assert part_2(TEST_INPUT) == 24
