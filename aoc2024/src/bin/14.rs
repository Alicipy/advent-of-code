use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

struct InputLine {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

struct Simulated {
    x: i64,
    y: i64,
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<InputLine>> {
    let lines = reader.lines();
    let mut p = vec![];
    for l in lines {
        let l = l?;
        let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").expect("bad regex");
        let res = regex.captures(l.as_str()).unwrap();
        let px = res[1].parse::<i64>()?;
        let py = res[2].parse::<i64>()?;
        let vx = res[3].parse::<i64>()?;
        let vy = res[4].parse::<i64>()?;
        p.push(InputLine { px, py, vx, vy });
    }

    Ok(p)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let input = parse_input(reader)?;

        let (width, height) = if input.len() > 20 {
            (101, 103)
        } else {
            (11, 7)
        };

        let simulations = input
            .iter()
            .map(|l| {
                let x = (((l.px + 100 * l.vx) % width) + width) % width;
                let y = (((l.py + 100 * l.vy) % height) + height) % height;
                Simulated { x, y }
            })
            .collect::<Vec<Simulated>>();

        let quadrants = simulations
            .iter()
            .map(|simulated| {
                (
                    (
                        simulated.x.cmp(&(width / 2)),
                        simulated.y.cmp(&(height / 2)),
                    ),
                    1,
                )
            })
            .filter(|((x, y), _b)| *x != Ordering::Equal && *y != Ordering::Equal)
            .into_group_map();
        let result = quadrants.values().map(|y| y.iter().sum())
            .reduce(|a, b| a * b)
            .unwrap();

        Ok(result)
    }

    assert_eq!(12, part1(BufReader::new(TEST1.as_bytes()))?);

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
