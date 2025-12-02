from aoc2025.utils import get_function_for_day_and_part, parse_args, read_input_for_day


def main(argv: list[str] | None = None) -> None:
    args = parse_args(argv)

    print(f"Executing day {args.day} part {args.part} ...")

    input_lines = read_input_for_day(args.day)
    func = get_function_for_day_and_part(args.day, args.part)

    result = func(input_lines)
    print(f"Result: {result}")


if __name__ == "__main__":
    raise SystemExit(main())
