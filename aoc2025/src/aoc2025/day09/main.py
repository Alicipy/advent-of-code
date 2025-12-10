import dataclasses
import sys
from copy import deepcopy


@dataclasses.dataclass(eq=True, frozen=True, order=True)
class Tile:
    x: int
    y: int


@dataclasses.dataclass(eq=True, frozen=True, order=True)
class CompressedTile:
    map_x: int
    map_y: int
    tile: Tile


def parse(lines: list[str]) -> list[Tile]:
    return [Tile(*map(int, line.split(","))) for line in lines]


def all_areas_trivial(tiles: list[Tile]) -> list[int]:
    areas = []
    for tile_a in tiles:
        for tile_b in tiles:
            areas.append((abs(tile_a.x - tile_b.x) + 1) * (abs(tile_a.y - tile_b.y) + 1))

    return areas


def compress(tiles: list[Tile]) -> list[CompressedTile]:
    x_coords = sorted(set(map(lambda tile: tile.x, tiles)))
    x_maps = {x: 2 * i + 1 for i, x in enumerate(x_coords)}
    y_coords = sorted(set(map(lambda tile: tile.y, tiles)))
    y_maps = {y: 2 * i + 1 for i, y in enumerate(y_coords)}

    compressed_tiles = [CompressedTile(x_maps[tile.x], y_maps[tile.y], tile) for tile in tiles]
    return compressed_tiles


def generate_ground_floor(compressed_tiles: list[CompressedTile]) -> list[list[bool]]:
    max_x = max(map(lambda tile: tile.map_x, compressed_tiles))
    max_y = max(map(lambda tile: tile.map_y, compressed_tiles))
    floor = [[False for _ in range(max_y + 2)] for _ in range(max_x + 2)]
    return floor


def _fill_outside(floor: list[list[bool]]) -> None:
    sys.setrecursionlimit(100000)

    def _fill_recursive(floor: list[list[bool]], pos: tuple[int, int]) -> None:
        x, y = pos
        if x < 0 or y < 0 or x >= len(floor) or y >= len(floor[0]):
            return
        if floor[x][y]:
            return
        floor[x][y] = True
        for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
            _fill_recursive(floor, (x + dx, y + dy))

    _fill_recursive(floor, (0, 0))


def fill_in_between(floor: list[list[bool]]) -> list[list[bool]]:
    floor = deepcopy(floor)
    fill_floor = deepcopy(floor)
    _fill_outside(fill_floor)

    for x in range(len(floor)):
        for y in range(len(floor[0])):
            if not fill_floor[x][y]:
                floor[x][y] = True
    return floor


def generate_tile_floor(compressed_tiles: list[CompressedTile]) -> list[list[bool]]:
    floor = generate_ground_floor(compressed_tiles)

    for fr, to in zip(compressed_tiles, compressed_tiles[1:] + compressed_tiles[:1]):
        assert fr.map_x == to.map_x or fr.map_y == to.map_y, "Tiles must be adjacent"
        lower_x, upper_x = min(fr.map_x, to.map_x), max(fr.map_x, to.map_x)
        lower_y, upper_y = min(fr.map_y, to.map_y), max(fr.map_y, to.map_y)
        for i in range(lower_y, upper_y + 1):
            floor[lower_x][i] = True
        for i in range(lower_x, upper_x + 1):
            floor[i][lower_y] = True

    floor = fill_in_between(floor)
    return floor


def allowed_areas(tile_map: list[list[bool]], compressed_tiles: list[CompressedTile]) -> list[int]:
    results = []

    for i, tile_a in enumerate(compressed_tiles):
        for tile_b in compressed_tiles[i + 1 :]:
            x_a, x_b = tile_a.map_x, tile_b.map_x
            y_a, y_b = tile_a.map_y, tile_b.map_y

            small_x, big_x = min(x_a, x_b), max(x_a, x_b)
            small_y, big_y = min(y_a, y_b), max(y_a, y_b)

            valid = True

            for x in range(small_x, big_x + 1):
                for y in range(small_y, big_y + 1):
                    if not tile_map[x][y]:
                        valid = False
                        break
                if not valid:
                    break

            if valid:
                results.append(
                    (abs(tile_a.tile.x - tile_b.tile.x) + 1)
                    * (abs(tile_a.tile.y - tile_b.tile.y) + 1)
                )

    return results


def part_1(lines: list[str]) -> int | str:
    tiles = parse(lines)
    areas = all_areas_trivial(tiles)
    return max(areas)


def part_2(lines: list[str]) -> int | str:
    tiles = parse(lines)
    compressed_tiles = compress(tiles)
    tile_map = generate_tile_floor(compressed_tiles)
    areas = allowed_areas(tile_map, compressed_tiles)
    return max(areas)
