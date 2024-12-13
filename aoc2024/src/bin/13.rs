use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

const NO_SOL: u64 = 100000;
fn simulate(p: (u64, u64), q: (u64, u64), r: (u64, u64)) -> Option<u64> {
    let mut res = NO_SOL;
    for i in 0..101 {
        for j in 0..101 {
            if i * p.0 + j * q.0 == r.0 && i * p.1 + j * q.1 == r.1 {
                res = min(res, i * 3 + j);
            }
        }
    }

    if res != NO_SOL {
        Some(res)
    } else {
        None
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let lines = reader.lines();
        let mut p = vec![];
        for l in lines {
            let l = l?;
            let regex = Regex::new(r"X.?(\d+), Y.?(\d+)$").expect("bad regex");
            let res = regex.captures(l.as_str());
            if let Some(x) = res {
                let n1 = x[1].parse::<u64>()?;
                let n2 = x[2].parse::<u64>()?;
                p.push((n1, n2));
            };
        }

        let mut result = 0;
        for i in (0..p.len()).step_by(3) {
            let res = simulate(p[i], p[i + 1], p[i + 2]);
            if let Some(x) = res {
                result += x
            }
        }

        Ok(result)
    }

    assert_eq!(480, part1(BufReader::new(TEST1.as_bytes()))?);

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
