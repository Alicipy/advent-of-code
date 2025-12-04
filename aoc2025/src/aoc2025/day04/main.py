def iterate(lines: list[list[str]]):
    removed = 0

    for x, line in enumerate(lines):
        for y, char in enumerate(line):
            surroundings = 0
            if char == ".":
                continue
            for dx in [-1, 0, 1]:
                for dy in [-1, 0, 1]:
                    if dx == 0 and dy == 0:
                        continue
                    new_x, new_y = x + dx, y + dy
                    if 0 <= new_x < len(lines) and 0 <= new_y < len(lines[new_x]):
                        char = lines[new_x][new_y]
                        surroundings += char == "@" or char == "X"

            if surroundings < 4:
                removed += 1
                lines[x][y] = "X"

    for line in lines:
        for i, char in enumerate(line):
            if char == "X":
                line[i] = "."

    return removed


def part_1(lines: list[str]) -> int | str:
    lines = [list(line) for line in lines]
    result = iterate(lines)
    return result


def part_2(lines: list[str]) -> int | str:
    lines = [list(line) for line in lines]
    result = 0
    while True:
        removed_rolls = iterate(lines)
        if not removed_rolls:
            break
        result += removed_rolls

    return result
