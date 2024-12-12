use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

const TEST2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

const TEST3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const TEST4: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

const TEST5: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

#[derive(Hash, PartialEq, Copy, Clone, Debug, Eq)]
enum NeighbourType {
    Top,
    Bottom,
    Left,
    Right,
}

const NEIGHBOURS: [((i32, i32), NeighbourType); 4] = [
    ((-1, 0), NeighbourType::Top),
    ((1, 0), NeighbourType::Bottom),
    ((0, -1), NeighbourType::Left),
    ((0, 1), NeighbourType::Right),
];

fn get_neighbour_pos(pos: (i32, i32)) -> Vec<((i32, i32), NeighbourType)> {
    let mut res = vec![];
    for (n, d) in NEIGHBOURS {
        let nx = pos.0 + n.0;
        let ny = pos.1 + n.1;

        res.push(((nx, ny), d));
    }
    res
}

fn colorize(
    visited: &mut Vec<Vec<u64>>,
    field: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    color: u64,
    exp_char: char,
) {
    if x < 0 || y < 0 || x >= visited.len() as i32 || y >= visited[0].len() as i32 {
        return;
    }
    let pos_x = x as usize;
    let pos_y = y as usize;

    if field[pos_x][pos_y] != exp_char {
        return;
    }

    if visited[pos_x][pos_y] != 0 {
        return;
    }
    visited[pos_x][pos_y] = color;

    for ((p, q), _) in get_neighbour_pos((x, y)) {
        colorize(visited, field, p, q, color, exp_char);
    }
}

fn read_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    let lines = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    lines
}

fn group_field(field: Vec<Vec<char>>) -> HashMap<u64, HashSet<(i32, i32)>> {
    let mut visited = field
        .iter()
        .map(|l| vec![0; l.len()])
        .collect::<Vec<Vec<u64>>>();

    let mut color = 1;
    for i in 0..field.len() {
        for (j, c) in field[i].iter().enumerate() {
            colorize(&mut visited, &field, i as i32, j as i32, color, *c);
            color += 1;
        }
    }

    let mut groups = HashMap::new();
    for (i, l) in visited.into_iter().enumerate() {
        for (j, v) in l.into_iter().enumerate() {
            groups
                .entry(v)
                .or_insert(HashSet::new())
                .insert((i as i32, j as i32));
        }
    }
    groups
}

#[derive(Clone, Debug, Hash)]
struct Fence {
    pos: (i32, i32),
    kind: NeighbourType,
}

fn get_fences(g: &HashSet<(i32, i32)>) -> Vec<Fence> {
    let mut fences = vec![];
    for elem in g {
        for (n, d) in get_neighbour_pos(*elem) {
            if !g.contains(&n) {
                let fence_position = (n.0 + elem.0, n.1 + elem.1);
                fences.push(Fence {
                    pos: fence_position,
                    kind: d,
                });
            }
        }
    }
    fences
}

fn count_sides_from_fences(fences: Vec<Fence>) -> usize {
    let mut sides = 0;

    for (r, grouped_fences) in fences.into_iter().into_group_map_by(|k| k.kind) {
        let aligned_groups = grouped_fences.into_iter().into_group_map_by(|f| match r {
            NeighbourType::Top | NeighbourType::Bottom => f.pos.0,
            NeighbourType::Left | NeighbourType::Right => f.pos.1,
        });

        for aligned_fences in aligned_groups.into_values() {
            let number_sides = aligned_fences
                .into_iter()
                .sorted_by(|a, b| a.pos.cmp(&b.pos))
                .tuple_windows::<(Fence, Fence)>()
                .filter(|(a, b)| (a.pos.0 - b.pos.0).abs() + (a.pos.1 - b.pos.1).abs() != 2)
                .count()
                + 1;
            sides += number_sides;
        }
    }

    sides
}

fn solve<R: BufRead>(reader: R, is_part_2: bool) -> Result<u64> {
    let lines = read_input(reader);

    let groups = group_field(lines);

    let mut result = 0;
    for g in groups.values() {
        let fences = get_fences(g);
        let surrounding = if is_part_2 {
            count_sides_from_fences(fences)
        } else {
            fences.len()
        };
        result += surrounding as u64 * g.len() as u64;
    }

    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        solve(reader, false)
    }

    assert_eq!(140, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(TEST3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        solve(reader, true)
    }

    assert_eq!(80, part2(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(436, part2(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(1206, part2(BufReader::new(TEST3.as_bytes()))?);
    assert_eq!(368, part2(BufReader::new(TEST4.as_bytes()))?);
    assert_eq!(236, part2(BufReader::new(TEST5.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
