use adv_code_2024::*;
use anyhow::*;
use cached::proc_macro::cached;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

#[cached]
fn simulate(number: u64, remaining_steps: u32) -> u64 {
    if remaining_steps == 0 {
        return 1;
    }
    let remaining_steps = remaining_steps - 1;
    match number {
        0 => simulate(1, remaining_steps),
        x => {
            let x_string = x.to_string();
            let len = x_string.len();
            if len % 2 == 0 {
                let left_num = x_string[..(len / 2)].parse::<u64>().unwrap();
                let right_num = x_string[(len / 2)..].parse::<u64>().unwrap();
                simulate(left_num, remaining_steps) + simulate(right_num, remaining_steps)
            } else {
                simulate(x * 2024, remaining_steps)
            }
        }
    }
}

fn solve<R: BufRead>(mut reader: R, steps: u32) -> Result<u64>{

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let numbers = buffer
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let result = numbers.iter().map(|f| simulate(*f, steps)).sum::<u64>();

    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        solve(reader, 25)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

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
