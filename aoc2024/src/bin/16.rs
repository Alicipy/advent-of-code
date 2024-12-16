use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

const TEST2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>> {
    let labyrinth = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok(labyrinth)
}

fn search_in_lab(labyrinth: &[Vec<char>], needle: char) -> (usize, usize) {
    for (i, line) in labyrinth.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == needle {
                return (i, j);
            }
        }
    }
    panic!("at the disco again")
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Reindeer {
    position: (usize, usize),
    orientation: u8,
}

const MOVEMENT_VECTOR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn traverse_labyrinth(
    labyrinth: &[Vec<char>],
    start_reindeer: Reindeer,
    end_reindeer: Reindeer,
) -> Option<u32> {
    let mut pq = PriorityQueue::new();

    // not the type one should be proud of.
    let mut visited = vec![vec![vec![None::<u32>; 4]; labyrinth[0].len()]; labyrinth.len()];

    pq.push(start_reindeer, Reverse(0));

    while !pq.is_empty() {
        let (reindeer, Reverse(prio)) = pq.pop().unwrap();
        let Reindeer {
            position,
            orientation,
        } = reindeer;

        if reindeer == end_reindeer {
            return Some(prio);
        }

        if labyrinth[position.0][position.1] == '#' {
            continue;
        }

        if visited[position.0][position.1][orientation as usize].is_some() {
            continue;
        }

        visited[position.0][position.1][orientation as usize] = Some(prio);

        let dir = MOVEMENT_VECTOR[orientation as usize];

        // turn left
        pq.push_increase(
            Reindeer {
                position,
                orientation: (orientation + 3) % 4,
            },
            Reverse(prio + 1000),
        );
        // turn right
        pq.push_increase(
            Reindeer {
                position,
                orientation: (orientation + 1) % 4,
            },
            Reverse(prio + 1000),
        );
        // step
        pq.push_increase(
            Reindeer {
                position: (
                    (position.0 as i32 + dir.0) as usize,
                    (position.1 as i32 + dir.1) as usize,
                ),
                orientation,
            },
            Reverse(prio + 1),
        );
    }

    None
}

fn construct_part1_min(
    labyrinth: &[Vec<char>],
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> Result<u32> {
    let start_reindeer = Reindeer {
        position: start_pos,
        orientation: 1,
    };

    let min_cost = (0..4)
        .map(|i| {
            traverse_labyrinth(
                labyrinth,
                start_reindeer,
                Reindeer {
                    position: end_pos,
                    orientation: i,
                },
            )
            .unwrap()
        })
        .min()
        .unwrap();

    Ok(min_cost)
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let labyrinth = parse_input(reader)?;

        let start_pos = search_in_lab(&labyrinth, 'S');
        let end_pos = search_in_lab(&labyrinth, 'E');

        construct_part1_min(&labyrinth, start_pos, end_pos)
    }

    assert_eq!(7036, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let labyrinth = parse_input(reader)?;

        let start_pos = search_in_lab(&labyrinth, 'S');
        let end_pos = search_in_lab(&labyrinth, 'E');

        let start_reindeer = Reindeer {
            position: start_pos,
            orientation: 1,
        };

        let min_cost = construct_part1_min(&labyrinth, start_pos, end_pos)?;

        let mut result_map = vec![vec![false; labyrinth[0].len()]; labyrinth.len()];

        for (i, row) in labyrinth.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                println!("{}, {}", i, j);
                if *c == '#' {
                    continue;
                }
                'tile_check: for k in 0..4 {
                    let temp_reindeer_pos = Reindeer {
                        position: (i, j),
                        orientation: k,
                    };

                    let start_to_temp_shortest =
                        traverse_labyrinth(&labyrinth, start_reindeer, temp_reindeer_pos).unwrap();

                    for l in 0..4 {
                        let end_reindeer_pos = Reindeer {
                            position: end_pos,
                            orientation: l,
                        };

                        let temp_to_end_shortest =
                            traverse_labyrinth(&labyrinth, temp_reindeer_pos, end_reindeer_pos)
                                .unwrap();

                        if start_to_temp_shortest + temp_to_end_shortest == min_cost {
                            result_map[i][j] = true;
                            break 'tile_check;
                        }
                    }
                }
            }
        }

        let num_fields = result_map.iter().flatten().filter(|&&x| x).count();

        Ok(num_fields)
    }

    assert_eq!(45, part2(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(64, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
