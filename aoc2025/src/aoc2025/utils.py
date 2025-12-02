import argparse
import importlib
import pathlib
import sys
from collections.abc import Callable


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run Advent of Code 2025 solutions.",
    )
    parser.add_argument(
        "-d",
        "--day",
        type=int,
        choices=list(range(1, 13)),
        required=True,
        help="Day number (1 - 12)",
    )
    parser.add_argument(
        "-p", "--part", type=int, choices=[1, 2], required=True, help="Part number (1 or 2)"
    )

    args = parser.parse_args(argv)
    return args


def read_input_for_day(day: int) -> list[str]:
    day = _format_day(day)
    path = pathlib.Path() / "data" / f"input_{day}"
    text = path.read_text().rstrip("\n")
    return text.splitlines()


def get_function_for_day_and_part(day: int, part: int) -> Callable[[list[str]], int | str]:
    day_str = _format_day(day)
    pkg = __package__ or ""
    module_name = f"{pkg + '.' if pkg else ''}day{day_str}.main"
    func_name = f"part_{part}"

    try:
        mod = importlib.import_module(module_name)
        func = getattr(mod, func_name)
        return func
    except AttributeError:
        print(f"Error: Function {func_name} not found in {module_name}.", file=sys.stderr)
        raise
    except ModuleNotFoundError:
        print(f"Error: Could not find module for day {day_str} ({module_name}).", file=sys.stderr)
        raise


def _format_day(day: int) -> str:
    if not (1 <= day <= 12):
        raise ValueError("day must be between 1 and 12")
    return f"{day:02d}"
