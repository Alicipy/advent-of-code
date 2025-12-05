import enum
from collections.abc import Generator
from dataclasses import dataclass


@dataclass(frozen=True, slots=True, kw_only=True)
class FreshIngredientRange:
    start: int
    end: int


@dataclass(frozen=True, slots=True, kw_only=True)
class Ingredient:
    value: int

    def is_between(self, ingredient_range: FreshIngredientRange) -> bool:
        return ingredient_range.start <= self.value <= ingredient_range.end


class SweeplineImpact(enum.IntEnum):
    start = -1
    stop = 1


@dataclass(frozen=True, slots=True, kw_only=True, order=True)
class IngredientRangeSweeplinePoint:
    value: int
    impact: SweeplineImpact


def parse(lines: list[str]) -> tuple[list[FreshIngredientRange], list[Ingredient]]:
    ingredient_ranges = []
    ingredients = []

    for line in lines:
        line = line.strip()

        if not line:
            continue

        if "-" in line:
            start, end = map(int, line.split("-", maxsplit=1))
            ingredient_range = FreshIngredientRange(start=start, end=end)
            ingredient_ranges.append(ingredient_range)
        else:
            value = int(line)
            ingredients.append(Ingredient(value=value))

    return ingredient_ranges, ingredients


def count_fresh_available_ingredients(
    ingredient_ranges: list[FreshIngredientRange], ingredients: list[Ingredient]
) -> int:
    fresh = 0
    for ingredient in ingredients:
        for ingredient_range in ingredient_ranges:
            if ingredient.is_between(ingredient_range):
                fresh += 1
                break

    return fresh


def _is_new_fresh_ingredient_range(
    active_count: int, sweepline_point: IngredientRangeSweeplinePoint
) -> bool:
    return active_count == SweeplineImpact.start and sweepline_point.impact == SweeplineImpact.start


def _is_end_of_fresh_ingredient_range(
    active_count: int, sweepline_point: IngredientRangeSweeplinePoint
) -> bool:
    return active_count == 0 and sweepline_point.impact == SweeplineImpact.stop


def optimize_fresh_ingredient_ranges(
    ingredient_ranges: list[FreshIngredientRange],
) -> Generator[FreshIngredientRange, None, None]:
    sweepline_points = []
    for ingredient_range in ingredient_ranges:
        sweepline_points.append(
            IngredientRangeSweeplinePoint(
                value=ingredient_range.start, impact=SweeplineImpact.start
            )
        )
        sweepline_points.append(
            IngredientRangeSweeplinePoint(value=ingredient_range.end, impact=SweeplineImpact.stop)
        )

    active_count = 0
    sweepline_points.sort()
    started = None
    for sweepline_point in sweepline_points:
        active_count += sweepline_point.impact

        if _is_new_fresh_ingredient_range(active_count, sweepline_point):
            assert started is None
            started = sweepline_point.value

        if _is_end_of_fresh_ingredient_range(active_count, sweepline_point):
            yield FreshIngredientRange(start=started, end=sweepline_point.value)
            started = None


def part_1(lines: list[str]) -> int | str:
    ingredient_ranges, ingredients = parse(lines)
    result = count_fresh_available_ingredients(ingredient_ranges, ingredients)
    return result


def part_2(lines: list[str]) -> int | str:
    ingredient_ranges, _ = parse(lines)
    ingredient_ranges = optimize_fresh_ingredient_ranges(ingredient_ranges)
    result = sum(map(lambda r: r.end - r.start + 1, ingredient_ranges))
    return result
