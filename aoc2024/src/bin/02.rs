use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

struct Input {
    lines: Vec<Vec<i64>>,
}

fn parse_input<R: BufRead>(reader: R) -> Result<Input> {
    let mut lines = vec![];

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<_> = line
            .split(" ")
            .map(|s| s.to_string().parse::<i64>().unwrap())
            .collect();
        lines.push(numbers);
    }

    let input = Input { lines };

    Ok(input)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = parse_input(reader)?;

        let mut result = 0;

        for line in input.lines {
            let sliding_window = line.windows(2);
            let differences: Vec<_> = sliding_window.map(|w| w[1] - w[0]).collect();
            //println!("{:#?}", differences);
            let all_up = differences.iter().all(|x| 0 < *x && *x < 4);
            let all_down = differences.iter().all(|x| -4 < *x && *x < 0);

            result += (all_up || all_down) as usize;
        }

        Ok(result)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = parse_input(reader)?;

        let mut result = 0;

        for line in input.lines {
            // hint: removing the first or last element does never break a safe list, so we
            // do not need a special case for lists safe without removing an element.
            for i in 0..line.len() {
                let mut variation = line.clone();
                variation.remove(i);

                let sliding_window = variation.windows(2);
                let differences: Vec<_> = sliding_window.map(|w| w[1] - w[0]).collect();
                //println!("{:#?}", differences);
                let all_up = differences.iter().all(|x| 0 < *x && *x < 4);
                let all_down = differences.iter().all(|x| -4 < *x && *x < 0);

                if all_up || all_down {
                    result += 1;
                    break;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
