use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use priority_queue::PriorityQueue;
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

fn search_start_pos(labyrinth: &[Vec<char>]) -> (usize, usize) {
    for (i, line) in labyrinth.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
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

fn is_end_pos(labyrinth: &[Vec<char>], pos: &(usize, usize)) -> bool {
    labyrinth[pos.0][pos.1] == 'E'
}

const MOVEMENT_VECTOR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let labyrinth = parse_input(reader)?;

        let start_reindeer = Reindeer {
            position: search_start_pos(&labyrinth),
            orientation: 1,
        };

        let mut pq = PriorityQueue::new();
        pq.push(start_reindeer, 0);

        while !pq.is_empty() {
            let (reindeer, prio) = pq.pop().unwrap();
            let Reindeer {
                position,
                orientation,
            } = reindeer;

            if labyrinth[position.0][position.1] == '#' {
                continue;
            }

            if is_end_pos(&labyrinth, &position) {
                return Ok(-prio);
            }

            let dir = MOVEMENT_VECTOR[orientation as usize];

            // turn left
            pq.push_increase(
                Reindeer {
                    position,
                    orientation: (orientation + 3) % 4,
                },
                prio - 1000,
            );
            // turn right
            pq.push_increase(
                Reindeer {
                    position,
                    orientation: (orientation + 1) % 4,
                },
                prio - 1000,
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
                prio - 1,
            );
        }

        panic!("No way found to to end?");
    }

    assert_eq!(7036, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST2.as_bytes()))?);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pq_new_is_higher_get_new() {
        let mut pq = priority_queue::PriorityQueue::new();
        pq.push_increase(1, -5);
        pq.push_increase(1, -4);

        let elem = pq.pop().unwrap();
        assert_eq!(elem.1, -4);
    }

    #[test]
    fn test_pq_new_is_lower_get_old() {
        let mut pq = priority_queue::PriorityQueue::new();
        pq.push_increase(1, -5);
        pq.push_increase(1, -6);

        let elem = pq.pop().unwrap();
        assert_eq!(elem.1, -5);
    }
}
