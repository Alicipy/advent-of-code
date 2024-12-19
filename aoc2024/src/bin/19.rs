use adv_code_2024::*;
use anyhow::*;
use cached::proc_macro::cached;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn parse_input<R: BufRead>(reader: R) -> Result<(Vec<String>, Vec<String>)> {
    let mut available_patterns = vec![];
    let mut requested = vec![];
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        } else if line.contains(",") {
            available_patterns.extend(line.split(",").map(|l| l.trim().to_string()));
        } else {
            requested.push(line);
        }
    }
    Ok((available_patterns, requested))
}

fn check_possible(pattern: String, available_patterns: Vec<String>) -> u64 {
    #[cached]
    fn _check_possible(request: String, available_patterns: Vec<String>, idx: usize) -> u64 {
        if idx >= request.len() {
            return 1;
        };
        let relevant_rest = request[idx..].to_string();

        let result = available_patterns
            .iter()
            .map(|pattern| {
                if relevant_rest.starts_with(pattern) {
                    _check_possible(
                        request.to_string(),
                        available_patterns.clone(),
                        idx + pattern.len(),
                    )
                } else {
                    0
                }
            })
            .sum();
        result
    }

    _check_possible(pattern, available_patterns, 0)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let (available_patterns, requested) = parse_input(reader)?;

        let num_possible = requested
            .into_iter()
            .map(|r| check_possible(r, available_patterns.clone()))
            .filter(|b| *b > 0)
            .count() as u64;
        Ok(num_possible)
    }

    assert_eq!(6, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let (available_patterns, requested) = parse_input(reader)?;

        let num_possible = requested
            .into_iter()
            .map(|r| check_possible(r, available_patterns.clone()))
            .sum::<u64>();
        Ok(num_possible)
    }

    assert_eq!(16, part2(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
