use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

struct Input {
    first_col: Vec<i64>,
    second_col: Vec<i64>,
}

fn parse_input<R: BufRead>(reader: R) -> Result<Input> {
    let mut input = Input {
        first_col: vec![],
        second_col: vec![],
    };

    for line in reader.lines() {
        let line = line?;
        let parts = line.split_once(" ").unwrap();
        println!("{:#?}", parts);
        let num1 = parts.0.parse::<i64>()?;
        let num2 = parts.1.trim_start().parse::<i64>()?;
        input.first_col.push(num1);
        input.second_col.push(num2);
    }

    Ok(input)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let mut input = parse_input(reader)?;
        input.first_col.sort();
        input.second_col.sort();

        let answer = zip(input.first_col, input.second_col)
            .map(|(a, b)| (b - a).abs())
            .sum();

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let input = parse_input(reader)?;
        let mut answer = 0i64;
        for num in input.first_col {
            let count: i64 = input
                .second_col
                .iter()
                .filter(|a| num == **a)
                .map(|_| 1)
                .sum();
            answer += num * count;
        }

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
