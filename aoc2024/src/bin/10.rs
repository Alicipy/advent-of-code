use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

const NEIGHBOURS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn read_input<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    let input = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    input
}

fn traverse(cur_pos: (i32, i32), field: &Vec<Vec<i32>>, expected_val: i32) -> Vec<(i32, i32)> {
    if cur_pos.0 < 0
        || cur_pos.1 < 0
        || cur_pos.0 >= field.len() as i32
        || cur_pos.1 >= field[0].len() as i32
    {
        return vec![];
    }
    if field[cur_pos.0 as usize][cur_pos.1 as usize] != expected_val {
        return vec![];
    }

    if expected_val == 9 {
        return vec![(cur_pos.0, cur_pos.1)];
    }

    let mut points = vec![];
    for n in NEIGHBOURS {
        let s = traverse(
            (cur_pos.0 + n[0], cur_pos.1 + n[1]),
            field,
            expected_val + 1,
        );
        points.extend(s.iter());
    }

    points
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn sol<R: BufRead>(reader: R, unique: bool) -> Result<usize> {
        let input = read_input(reader);

        let mut res = 0;
        for (i, a) in input.iter().enumerate() {
            for j in 0..a.iter().len() {
                let mut traversal_result = traverse((i as i32, j as i32), &input, 0);
                if unique {
                    traversal_result = traversal_result.iter().unique().copied().collect();
                }
                res += traversal_result.len();
            }
        }

        Ok(res)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        sol(reader, true)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        sol(reader, false)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
