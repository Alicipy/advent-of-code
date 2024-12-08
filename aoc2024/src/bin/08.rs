use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

type Point = (i32, i32);

fn calc_point_before(a: &Point, b: &Point) -> Point {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;

    (a.0 - dx, a.1 - dy)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut input = vec![];
        let mut input_map = HashMap::new();
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            input.push(line.clone());
            for (j, ch) in line.chars().enumerate() {
                if ch != '.' {
                    input_map
                        .entry(ch)
                        .or_insert(Vec::new())
                        .push((i as i32, j as i32));
                }
            }
        }

        let mut calc_points = HashSet::new();

        for (_, points) in input_map.iter() {
            points.iter().permutations(2).for_each(|perm| {
                let a = perm[0];
                let b = perm[1];
                let p1 = calc_point_before(a, b);
                let p2 = calc_point_before(b, a);
                calc_points.insert(p1);
                calc_points.insert(p2);
            })
        }

        let in_bound = calc_points.iter().filter(|p| {
            0 <= p.0 && p.0 < input.len() as i32 && 0 <= p.1 && p.1 < input[0].len() as i32
        });
        let result = in_bound.count();

        Ok(result)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

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
