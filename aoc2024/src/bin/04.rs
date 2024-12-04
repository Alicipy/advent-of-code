use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const SEARCH_EXPR_1: [char; 4] = ['X', 'M', 'A', 'S'];

fn read_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    let input: Vec<_> = reader
        .lines()
        .map(|x| x.unwrap().to_string().chars().collect::<Vec<_>>())
        .collect();
    input
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let input: Vec<_> = read_input(reader);
        let mut result = 0;
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                for xd in -1..2 {
                    for yd in -1..2 {
                        let mut found = true;
                        for (d, c) in SEARCH_EXPR_1.iter().enumerate() {
                            let x = i as i32 + xd * d as i32;
                            let y = j as i32 + yd * d as i32;
                            found &= 0 <= x
                                && x < input.len() as i32
                                && 0 <= y
                                && y < input[i].len() as i32
                                && input[x as usize][y as usize] == *c;
                        }
                        result += found as u32;
                    }
                }
            }
        }

        Ok(result)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let input: Vec<_> = read_input(reader);
        let mut result = 0;
        for i in 1..input.len() - 1 {
            for j in 1..input[i].len() - 1 {
                let w1 = String::from_iter([input[i - 1][j - 1], input[i][j], input[i + 1][j + 1]]);
                let w2 = String::from_iter([input[i + 1][j - 1], input[i][j], input[i - 1][j + 1]]);

                result += ((w1 == "MAS" || w1 == "SAM") && (w2 == "MAS" || w2 == "SAM")) as u32;
            }
        }

        Ok(result)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
