use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

struct Input {
    expected_answer: u64,
    numbers: Vec<u64>,
}

fn read_input<R: BufRead>(reader: R) -> Vec<Input> {
    let inputs: Vec<_> = reader
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let (a, b) = x.split_once(":").unwrap();
            let expected_answer = a.parse::<u64>().unwrap();
            let numbers: Vec<u64> = b
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            Input {
                expected_answer,
                numbers,
            }
        })
        .collect();
    inputs
}

fn validate(input: Input, combination_function: fn(u64, u64) -> Vec<u64>) -> bool {
    fn recurse(vec: Vec<u64>, combination_function: fn(u64, u64) -> Vec<u64>) -> Vec<u64> {
        if vec.len() == 1 {
            return vec;
        }

        let mut rest = vec.clone();
        let cur = rest.pop().unwrap();

        let recursive_results = recurse(rest, combination_function);
        let results = recursive_results
            .iter()
            .flat_map(|a| combination_function(*a, cur))
            .collect();

        results
    }

    recurse(input.numbers, combination_function).contains(&input.expected_answer)
}

fn solve<R: BufRead>(reader: R, combination_function: fn(u64, u64) -> Vec<u64>) -> Result<u64> {
    let inputs = read_input(reader);

    let mut answer = 0;
    for input in inputs {
        let potential_res = input.expected_answer;
        let res = validate(input, combination_function);
        if res {
            answer += potential_res;
        }
    }

    Ok(answer)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        fn combine(left: u64, right: u64) -> Vec<u64> {
            vec![left + right, left * right]
        }
        solve(reader, combine)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        fn combine(left: u64, right: u64) -> Vec<u64> {
            let cat = format!("{}{}", left, right).parse::<u64>().unwrap();
            vec![left + right, left * right, cat]
        }
        solve(reader, combine)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
