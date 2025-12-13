import dataclasses
from functools import cache

ME = "you"
SERVER = "svr"

OUT = "out"


@dataclasses.dataclass
class Node:
    identifier: str
    children: list["Node"] = dataclasses.field(default_factory=list)


def extract_node_names(lines: list[str]) -> list[list[str]]:
    return [line.replace(":", "").strip().split(" ") for line in lines]


def create_node_map(node_lines: list[list[str]]) -> dict[str, Node]:
    node_map = {}
    for node_line in node_lines:
        for node_name in node_line:
            node_map[node_name] = Node(node_name)
    return node_map


def add_node_children(node_map: dict[str, Node], node_lines: list[list[str]]) -> None:
    for node_line in node_lines:
        parent, *children = node_line
        for child in children:
            node_map[parent].children.append(node_map[child])


def find_number_of_flows(
    node_map: dict[str, Node], start_node, expected_inbetween: set[str]
) -> int:
    @cache
    def _helper(current: str, found_inbetween: tuple[str, ...]):
        if current == OUT:
            return 1 if len(found_inbetween) == len(expected_inbetween) else 0

        res = 0
        if current in expected_inbetween:
            found_inbetween += (current,)
        node = node_map[current]
        for child in node.children:
            res += _helper(child.identifier, found_inbetween)
        return res

    return _helper(start_node, tuple())


def parse(lines: list[str]) -> dict[str, Node]:
    node_lines = extract_node_names(lines)
    node_map = create_node_map(node_lines)
    add_node_children(node_map, node_lines)

    return node_map


def part_1(lines: list[str]) -> int:
    node_map = parse(lines)
    return find_number_of_flows(node_map, ME, set())


def part_2(lines: list[str]) -> int | str:
    node_map = parse(lines)
    return find_number_of_flows(node_map, SERVER, {"fft", "dac"})
