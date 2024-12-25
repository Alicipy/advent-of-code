use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "25";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const LOCK_SIZE: usize = 7;

const TEST1: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<String>> {
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|p| !p.is_empty())
        .collect();

    // Zip locks and keys both into single string
    let mut elements = vec![];
    for i in (0..lines.len()).step_by(LOCK_SIZE) {
        elements.push(lines[i..(i + LOCK_SIZE)].join(""));
    }

    Ok(elements)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let elements = parse_input(reader)?;

        let mut result = 0;

        // Now check for overlaps for each pair. Locks and locks or keys and keys
        // will just fail during the first or last row, so no need to separate
        for e1 in elements.iter() {
            for e2 in elements.iter() {
                let overlap = e1
                    .chars()
                    .zip(e2.chars())
                    .any(|(a, b)| a == '#' && b == '#');
                if !overlap {
                    result += 1;
                }
            }
        }

        // we counted double as we checked each pair twice
        result /= 2;

        Ok(result)
    }

    assert_eq!(3, part1(BufReader::new(TEST1.as_bytes()))?);

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
