use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
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

fn traverse(cur_pos: (i32, i32), field: &Vec<Vec<i32>>, expected_val: i32) -> HashSet<(i32, i32)> {
    if cur_pos.0 < 0
        || cur_pos.1 < 0
        || cur_pos.0 >= field.len() as i32
        || cur_pos.1 >= field[0].len() as i32
    {
        return HashSet::new();
    }
    if field[cur_pos.0 as usize][cur_pos.1 as usize] != expected_val {
        return HashSet::new();
    }

    if expected_val == 9 {
        let mut set = HashSet::new();
        set.insert((cur_pos.0, cur_pos.1));
        return set;
    }

    let mut set: HashSet<_> = HashSet::new();
    for n in NEIGHBOURS {
        let s = traverse(
            (cur_pos.0 + n[0], cur_pos.1 + n[1]),
            field,
            expected_val + 1,
        );
        set = set.union(&s).copied().collect();
    }

    set
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
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

        let mut res = 0;
        for (i, a) in input.iter().enumerate() {
            for j in 0..a.iter().len() {
                res += traverse((i as i32, j as i32), &input, 0).len();
            }
        }

        Ok(res)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

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
