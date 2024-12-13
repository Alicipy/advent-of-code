use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
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

type Equation = (i64, i64, i64);

#[derive(Debug)]
struct EquationPair {
    first_eq: Equation,
    second_eq: Equation,
}

fn solve(eq_pair: EquationPair) -> Option<u64> {
    let cross_x = EquationPair {
        first_eq: (
            eq_pair.first_eq.0 * eq_pair.second_eq.0,
            eq_pair.first_eq.1 * eq_pair.second_eq.0,
            eq_pair.first_eq.2 * eq_pair.second_eq.0,
        ),
        second_eq: (
            eq_pair.second_eq.0 * eq_pair.first_eq.0,
            eq_pair.second_eq.1 * eq_pair.first_eq.0,
            eq_pair.second_eq.2 * eq_pair.first_eq.0,
        ),
    };

    let subtracted = (
        cross_x.first_eq.0 - cross_x.second_eq.0,
        cross_x.first_eq.1 - cross_x.second_eq.1,
        cross_x.first_eq.2 - cross_x.second_eq.2,
    );

    assert_eq!(subtracted.0, 0);

    if subtracted.2 % subtracted.1 != 0 {
        return None;
    }

    let y_sol = subtracted.2 / subtracted.1;

    if (eq_pair.first_eq.2 - (y_sol * eq_pair.first_eq.1)) % eq_pair.first_eq.0 != 0 {
        return None;
    }

    let x_sol = (eq_pair.first_eq.2 - (y_sol * eq_pair.first_eq.1)) / eq_pair.first_eq.0;

    assert_eq!(
        eq_pair.first_eq.0 * x_sol + eq_pair.first_eq.1 * y_sol,
        eq_pair.first_eq.2
    );
    assert_eq!(
        eq_pair.second_eq.0 * x_sol + eq_pair.second_eq.1 * y_sol,
        eq_pair.second_eq.2
    );

    if x_sol < 0 || y_sol < 0 {
        None
    } else {
        Some(x_sol as u64 * 3 + y_sol as u64)
    }
}

fn read_input<R: BufRead>(reader: R) -> Vec<EquationPair> {
    let lines = reader.lines();
    let mut p = vec![];
    for l in lines {
        let l = l.unwrap();
        let regex = Regex::new(r"X.?(\d+), Y.?(\d+)$").expect("bad regex");
        let res = regex.captures(l.as_str());
        if let Some(x) = res {
            let n1 = x[1].parse::<i64>().unwrap();
            let n2 = x[2].parse::<i64>().unwrap();
            p.push((n1, n2));
        };
    }

    p.into_iter()
        .tuples::<((i64, i64), (i64, i64), (i64, i64))>()
        .map(|(fl, sl, rl)| EquationPair {
            first_eq: (fl.0, sl.0, rl.0),
            second_eq: (fl.1, sl.1, rl.1),
        })
        .collect()
}

const PART_2_RESULT_OFFSET: i64 = 10000000000000;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let equations = read_input(reader);

        let result = equations
            .into_iter()
            .map(solve)
            .map(|r| r.unwrap_or(0))
            .sum();

        Ok(result)
    }

    assert_eq!(480, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let equations = read_input(reader);

        let offseted_equations: Vec<_> = equations
            .into_iter()
            .map(|eq| EquationPair {
                first_eq: (
                    eq.first_eq.0,
                    eq.first_eq.1,
                    eq.first_eq.2 + PART_2_RESULT_OFFSET,
                ),
                second_eq: (
                    eq.second_eq.0,
                    eq.second_eq.1,
                    eq.second_eq.2 + PART_2_RESULT_OFFSET,
                ),
            })
            .collect();

        let result = offseted_equations
            .into_iter()
            .map(solve)
            .map(|r| r.unwrap_or(0))
            .sum();

        Ok(result)
    }

    //assert_eq!(0, part2(BufReader::new(TEST1.as_bytes()))?); unknown so far...

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
