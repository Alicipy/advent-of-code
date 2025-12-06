from aoc2025.day06.main import interpret_right_to_left, parse, part_1, part_2

TEST_INPUT = """
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
""".lstrip().splitlines()


def test__parse():
    assert parse(TEST_INPUT) == (
        [["123", "328", " 51", "64 "], [" 45", "64 ", "387", "23 "], ["  6", "98 ", "215", "314"]],
        ["*", "+", "*", "+"],
    )


def test__interpret_left_to_right__first_col():
    assert interpret_right_to_left(["328", "64 ", "98 "]) == ["369", "248", "8"]


def test__interpret_left_to_right__second_col():
    assert interpret_right_to_left(["123", " 45", "  6"]) == ["1", "24", "356"]


def test__part_1__example():
    assert part_1(TEST_INPUT) == 4277556


def test__part_2__example():
    assert part_2(TEST_INPUT) == 3263827
