use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn search_in_labyrinth(labyrinth: &[Vec<char>], c: char) -> (usize, usize) {
    for (i, line) in labyrinth.iter().enumerate() {
        for (j, k) in line.iter().enumerate() {
            if *k == c {
                return (i, j);
            }
        }
    }
    panic!("could not find {}", c);
}

const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn calculate_dist_to_end(labyrinth: &[Vec<char>]) -> Vec<Vec<Option<u32>>> {
    let end_point = search_in_labyrinth(labyrinth, 'E');

    let mut distances = vec![vec![None; labyrinth.len()]; labyrinth[0].len()];

    let mut queue = VecDeque::new();
    queue.push_back(end_point);

    let mut distance = 0;

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();

        if labyrinth[next.0][next.1] == '#' || distances[next.0][next.1].is_some() {
            continue;
        }

        distances[next.0][next.1] = Some(distance);
        distance += 1;

        for n in NEIGHBOURS {
            let nx = next.0 as i32 + n.0;
            let ny = next.1 as i32 + n.1;

            if nx < 0 || ny < 0 || nx >= labyrinth.len() as i32 || ny >= labyrinth[0].len() as i32 {
                continue;
            }

            queue.push_back((nx as usize, ny as usize));
        }
    }

    distances
}

struct Cheat {
    _start_point: (usize, usize),
    _end_point: (usize, usize),
    saved_secs: u32,
}

fn calculate_cheats(distances: &[Vec<Option<u32>>]) -> Vec<Cheat> {
    let mut cheats = vec![];

    for (xs, line) in distances.iter().enumerate() {
        for (ys, start_pos) in line.iter().enumerate() {
            match start_pos {
                None => {}
                Some(start_distance) => {
                    for n in NEIGHBOURS {
                        let xe = xs as i32 + 2 * n.0;
                        let ye = ys as i32 + 2 * n.1;

                        if xe < 0
                            || ye < 0
                            || xe as usize >= distances.len()
                            || ye as usize >= distances[ys].len()
                        {
                            continue;
                        }

                        let xe = xe as usize;
                        let ye = ye as usize;

                        if let Some(end_distance) = distances[xe][ye] {
                            let saved_secs = start_distance
                                .saturating_sub(end_distance)
                                .saturating_sub(2);
                            if saved_secs == 0 {
                                continue;
                            }
                            let cheat = Cheat {
                                _start_point: (xs, ys),
                                _end_point: (xe, ye),
                                saved_secs,
                            };
                            cheats.push(cheat);
                        }
                    }
                }
            }
        }
    }

    cheats
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, minimal_save_dist: u32) -> Result<u64> {
        let labyrinth = parse_input(reader);

        let distances = calculate_dist_to_end(&labyrinth);

        let cheats = calculate_cheats(&distances);

        Ok(cheats
            .into_iter()
            .filter(|c| c.saved_secs >= minimal_save_dist)
            .count() as u64)
    }

    assert_eq!(8, part1(BufReader::new(TEST1.as_bytes()), 12)?);
    assert_eq!(5, part1(BufReader::new(TEST1.as_bytes()), 19)?);
    assert_eq!(5, part1(BufReader::new(TEST1.as_bytes()), 20)?);
    assert_eq!(4, part1(BufReader::new(TEST1.as_bytes()), 21)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 100)?);
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
