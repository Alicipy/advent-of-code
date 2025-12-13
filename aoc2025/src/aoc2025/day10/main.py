from functools import lru_cache

import numpy
from scipy.optimize import LinearConstraint, milp

INF = 10**9


def parse(line: str) -> tuple[int, list[list[int]], tuple[int, ...]]:
    squared, *brackets, curly = line.split()

    indicator = int(squared[1:-1:].replace(".", "0").replace("#", "1")[::-1], 2)
    wiring = []
    for bracket in brackets:
        current_wiring = []
        for wire in bracket[1:-1].split(","):
            current_wiring.append(int(wire))
        wiring.append(current_wiring)

    joltages = tuple(map(int, curly[1:-1].split(",")))

    return indicator, wiring, joltages


def to_numbers(wiring: list[list[int]]) -> list[int]:
    result = []
    for wiring_row in wiring:
        num = 0
        for wire in wiring_row:
            num += 1 << wire
        result.append(num)

    return result


def minimal_button_presses_lights(indicator, wiring) -> int:
    @lru_cache
    def _minimal_button_helper(current_indicator: int, pos: int) -> int:
        if pos >= len(wiring) and current_indicator != 0:
            return INF
        if current_indicator == 0:
            return 0

        current_button = wiring[pos]
        without_press_best = _minimal_button_helper(current_indicator, pos + 1)
        with_press_best = _minimal_button_helper(current_indicator ^ current_button, pos + 1) + 1

        return min(without_press_best, with_press_best)

    return _minimal_button_helper(indicator, 0)


def minimal_button_presses_joltages(wiring: list[list[int]], joltages: tuple[int, ...]) -> int:
    A = numpy.zeros((len(joltages), len(wiring)))
    for i, wire in enumerate(wiring):
        A[wire, i] = 1
    b = numpy.array(joltages)
    c = numpy.ones(len(wiring))

    integrality = numpy.ones_like(c)
    constraints = LinearConstraint(A, b, b)
    result = milp(c=c, constraints=constraints, integrality=integrality)
    return int(sum(result.x))


def part_1(lines: list[str]) -> int:
    result = 0
    for line in lines:
        indicator, wiring, _ = parse(line)
        numbered_wiring = to_numbers(wiring)
        result += minimal_button_presses_lights(indicator, numbered_wiring)

    return result


def part_2(lines: list[str]) -> int | str:
    result = 0
    for line in lines:
        _, wiring, joltages = parse(line)
        result += minimal_button_presses_joltages(wiring, joltages)

    return result
