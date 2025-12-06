import functools
import operator

type UnpaddedNumberStr = list[str]
type Operator = str


def parse(lines: list[str]) -> tuple[list[UnpaddedNumberStr], list[Operator]]:
    number_rows, operators = lines[:-1], lines[-1]
    assert len(lines[0]) == len(operators)
    cut_numbers = [i for i, c in enumerate(operators) if c != " "] + [len(operators) + 1]
    unpadded_num_rows = [[] for _ in number_rows]

    for fr, to in zip(cut_numbers[:-1], cut_numbers[1:]):
        for i, number_row in enumerate(number_rows):
            unpadded_num_rows[i].append(number_row[fr : to - 1])

    operators = operators.strip().split()
    return unpadded_num_rows, operators


def rows_to_cols(lines: list[UnpaddedNumberStr]) -> list[UnpaddedNumberStr]:
    return list(zip(*lines))


def interpret_right_to_left(col: UnpaddedNumberStr) -> UnpaddedNumberStr:
    transposed = list(zip(*col))
    col = ["".join(c).rstrip() for c in transposed]
    return col


def evaluate(col: UnpaddedNumberStr, op: Operator) -> int:
    col = list(map(int, col))
    if op == "*":
        return functools.reduce(operator.mul, col)
    if op == "+":
        return functools.reduce(operator.add, col)

    assert False, f"Unknown operator {op}"


def get_result(cols: list[UnpaddedNumberStr], ops: list[Operator]) -> int:
    result = 0
    for col, op in zip(cols, ops):
        result += evaluate(col, op)
    return result


def solve(lines: list[str], right_to_left_math: bool) -> int | str:
    number_rows, operators = parse(lines)
    number_cols = rows_to_cols(number_rows)
    if right_to_left_math:
        number_cols = [interpret_right_to_left(number_col) for number_col in number_cols]
    result = get_result(number_cols, operators)
    return result


def part_1(lines: list[str]) -> int | str:
    return solve(lines, False)


def part_2(lines: list[str]) -> int | str:
    return solve(lines, True)
