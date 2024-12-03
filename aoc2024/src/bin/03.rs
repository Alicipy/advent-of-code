use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Instr::{DO, DONT};
use regex::Regex;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

#[allow(clippy::upper_case_acronyms)]
enum Instr {
    DO,
    DONT,
    MUL(i64, i64),
}

struct InputMatch {
    pos: usize,
    instr: Instr,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let answer = reader.lines();
        let mut result = 0;
        for line in answer {
            let line = line?;
            let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("No valid regex given?");
            let res: i64 = regex
                .captures_iter(line.as_str())
                .map(|cap| {
                    let a = cap[1].parse::<i64>().expect("Cannot parse number");
                    let b = cap[2].parse::<i64>().expect("Cannot parse number");
                    a * b
                })
                .sum();
            result += res;
        }

        Ok(result)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let input = reader.lines();

        let do_regex = Regex::new(r"do\(\)")?;
        let dont_regex = Regex::new(r"don't\(\)")?;
        let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("No valid regex given?");

        let mut result = 0;
        let mut mode = 1;

        for line in input {
            let line = line?;
            let line = line.as_str();
            let mut linematches = vec![];
            do_regex.find_iter(line).for_each(|x| {
                linematches.push(InputMatch {
                    pos: x.start(),
                    instr: DO,
                })
            });
            dont_regex.find_iter(line).for_each(|x| {
                linematches.push(InputMatch {
                    pos: x.start(),
                    instr: DONT,
                })
            });
            mul_regex.captures_iter(line).for_each(|x| {
                let a = x[1].parse::<i64>().expect("Cannot parse number");
                let b = x[2].parse::<i64>().expect("Cannot parse number");
                let pos = x.get(0).unwrap().start();
                linematches.push(InputMatch {
                    pos,
                    instr: Instr::MUL(a, b),
                });
            });

            linematches.sort_by(|a, b| a.pos.cmp(&(b.pos)));

            for v in linematches {
                match v.instr {
                    DO => mode = 1,
                    DONT => mode = 0,
                    Instr::MUL(a, b) => result += a * b * mode,
                }
            }
        }

        Ok(result)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
