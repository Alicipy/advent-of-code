import dataclasses
from typing import Self


@dataclasses.dataclass(eq=True, frozen=True, order=True)
class JunctionBox:
    idx: int
    x: int
    y: int
    z: int

    def distance_to(self, other: Self) -> int:
        return (other.x - self.x) ** 2 + (other.y - self.y) ** 2 + (other.z - self.z) ** 2


@dataclasses.dataclass
class JunctionBoxDistance:
    distance: int
    box_a: JunctionBox
    box_b: JunctionBox


def parse(lines: list[str]) -> list[JunctionBox]:
    return [JunctionBox(i, *map(int, line.split(","))) for i, line in enumerate(lines)]


def generate_distances(boxes: list[JunctionBox]):
    distances = []
    for i, fr in enumerate(boxes):
        for to in boxes[i + 1 :]:
            dist = fr.distance_to(to)
            distances.append(JunctionBoxDistance(distance=dist, box_a=fr, box_b=to))

    return distances


def minimal_spanning_tree(
    boxes: list[JunctionBox], distances: list[JunctionBoxDistance], num_steps: int | None
) -> tuple[list[set[JunctionBox]], int]:
    spanning_tree_cluster = [{box} for box in boxes]
    distances.sort(key=lambda x: x.distance)
    steps = num_steps if num_steps else len(distances)

    for i, next_closest_pair in enumerate(distances[:steps]):
        a_set = spanning_tree_cluster[next_closest_pair.box_a.idx]
        b_set = spanning_tree_cluster[next_closest_pair.box_b.idx]
        unionized_set = a_set.union(b_set)
        for unionized_element in unionized_set:
            spanning_tree_cluster[unionized_element.idx] = unionized_set

        if len(unionized_set) == len(boxes):
            break

    return spanning_tree_cluster, i


def solve(lines: list[str], max_steps: int | None) -> int:
    boxes = parse(lines)
    distances = generate_distances(boxes)
    spanning_tree_circuits, index = minimal_spanning_tree(boxes, distances, max_steps)

    if max_steps is not None:
        unique_circuits = set(map(lambda x: (len(x), max(x)), spanning_tree_circuits))
        unique_circuits = sorted(unique_circuits, key=lambda x: x[0], reverse=True)
        return unique_circuits[0][0] * unique_circuits[1][0] * unique_circuits[2][0]

    else:
        box_a = distances[index].box_a
        box_b = distances[index].box_b

        return box_a.x * box_b.x


def part_1(lines: list[str]) -> int | str:
    return solve(lines, max_steps=1000)


def part_2(lines: list[str]) -> int | str:
    return solve(lines, max_steps=None)
