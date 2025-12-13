from aoc2025.day11.main import part_1, part_2

TEST_INPUT_1 = """
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
""".strip().splitlines()

TEST_INPUT_2 = """
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
""".strip().splitlines()


def test__part_1__solve_example():
    assert part_1(TEST_INPUT_1) == 5


def test__part_2__solve_example():
    assert part_2(TEST_INPUT_2) == 2
