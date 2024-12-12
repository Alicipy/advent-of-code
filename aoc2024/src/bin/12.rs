use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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

const NEIGHBOURS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn get_neighbour_pos(pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut res = vec![];
    for n in NEIGHBOURS {
        let nx = pos.0 + n[0];
        let ny = pos.1 + n[1];

        res.push((nx, ny));
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

    for (p, q) in get_neighbour_pos((x, y)) {
        colorize(visited, field, p, q, color, exp_char);
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let lines = reader
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut visited = lines
            .iter()
            .map(|l| vec![0; l.len()])
            .collect::<Vec<Vec<u64>>>();

        let mut color = 1;
        for i in 0..lines.len() {
            for (j, c) in lines[i].iter().enumerate() {
                colorize(&mut visited, &lines, i as i32, j as i32, color, *c);
                color += 1;
            }
        }

        let mut groups = HashMap::new();
        for (i, _) in visited.iter().enumerate() {
            for (j, _) in visited[i].iter().enumerate() {
                groups
                    .entry([visited[i][j]])
                    .or_insert(HashSet::new())
                    .insert((i as i32, j as i32));
            }
        }

        let mut result = 0;
        for g in groups.values() {
            let mut fences = 0;
            for elem in g {
                for n in get_neighbour_pos(*elem) {
                    if !g.contains(&n) {
                        fences += 1;
                    }
                }
            }
            result += fences * g.len() as u64;
        }

        Ok(result)
    }

    assert_eq!(140, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(TEST3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
