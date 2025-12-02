def solve(lines: list[str], part: int):
    pos = 50
    counts = 0

    for line in lines:
        sign = 1 if line[0] == "R" else -1
        number = int(line[1:])

        full_spins = number // 100
        number %= 100

        prev_pos = pos

        pos += sign * number
        if pos % 100 == 0:
            counts += 1

        if part == 2:
            # When you pass the 0, you come into another '100' section which we count.
            # Corner-case when you land at exactly
            # 0 from left or right, but this is handled by the default 'part_1' case already.
            if pos % 100 != 0 and prev_pos % 100 != 0 and pos // 100 != prev_pos // 100:
                counts += 1

            counts += full_spins

    return counts


def part_1(lines: list[str]) -> int | str:
    return solve(lines, 1)


def part_2(lines: list[str]) -> int | str:
    return solve(lines, 2)
