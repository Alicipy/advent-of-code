import dataclasses


@dataclasses.dataclass
class Block:
    block: list[str]

    def num_blocks(self) -> int:
        return sum(map(lambda t: len(list(filter(lambda b: b == "#", t))), self.block))


@dataclasses.dataclass
class PuzzleInput:
    size: tuple[int, int]
    presents_to_fit: list[int]


def parse(lines: list[str]) -> tuple[list[Block], list[PuzzleInput]]:
    blocks = []
    for i in range(6):
        _ = lines.pop(0)
        block = [lines.pop(0) for _ in range(3)]
        blocks.append(Block(block))
        lines.pop(0)

    puzzle_inputs = []
    for line in lines:
        sizes, presents = line.split(":")
        h, b = list(map(int, sizes.split("x")))
        presents_to_fit = list(map(int, presents.strip().split(" ")))
        puzzle_inputs.append(PuzzleInput(size=(b, h), presents_to_fit=presents_to_fit))

    return blocks, puzzle_inputs


def solve_single(blocks: list[Block], puzzle_input: PuzzleInput) -> bool:
    puzzle_area = puzzle_input.size[0] * puzzle_input.size[1]
    if sum(puzzle_input.presents_to_fit) * 9 <= puzzle_area:
        return True

    count_blocks = 0
    for block, present_num in zip(blocks, puzzle_input.presents_to_fit):
        count_blocks += block.num_blocks() * present_num

    if count_blocks > puzzle_area:
        return False

    assert False, "Non-trivial case to implement, not necessary? " + str(puzzle_input)


def part_1(lines: list[str]) -> int:
    blocks, puzzle_inputs = parse(lines)

    return len(list(filter(lambda puzzle_input: solve_single(blocks, puzzle_input), puzzle_inputs)))
