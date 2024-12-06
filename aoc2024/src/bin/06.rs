use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn find_start(field: &[Vec<char>]) -> Result<(usize, usize)> {
    for (i, s) in field.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if *c == '^' {
                let result = anyhow::Ok((i, j));
                return result;
            }
        }
    }
    Err(Error::msg("No match"))
}

const MAX_STEPS: u32 = 1_000_000;

const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn simulate(field: &[Vec<char>]) -> (Vec<Vec<bool>>, u32) {
    let start_pos = find_start(field).unwrap();
    let mut cur = (start_pos, 0);
    let mut visited = field
        .iter()
        .map(|v| v.iter().map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<_>>();

    let mut stepsize: u32 = 0;
    loop {
        stepsize += 1;

        let (pos, dir) = cur;
        visited[pos.0][pos.1] = true;

        let step = DIRECTIONS[dir];

        let nx = pos.0 as i32 + step[0];
        let ny = pos.1 as i32 + step[1];

        if nx < 0
            || ny < 0
            || nx >= field.len() as i32
            || ny >= field[0].len() as i32
            || stepsize == MAX_STEPS
        {
            return (visited, stepsize);
        }

        let nx = nx as usize;
        let ny = ny as usize;

        let nxt = field[nx][ny];
        if nxt == '#' {
            cur = (pos, (dir + 1) % 4)
        } else {
            cur = ((nx, ny), dir);
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn read_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
        let input: Vec<_> = reader
            .lines()
            .map(|x| x.unwrap().chars().collect::<Vec<_>>())
            .collect();
        input
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = read_input(reader);

        let (visited, _) = simulate(&input);

        let answer = visited.iter().flatten().map(|x| *x as usize).sum();
        Ok(answer)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = read_input(reader);

        let start_field = find_start(&input)?;

        let mut result = 0;
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if (i, j) == start_field {
                    continue;
                }
                let mut current_field = input.clone();
                current_field[i][j] = '#';

                let (_, steps) = simulate(&current_field);
                if steps == MAX_STEPS {
                    result += 1;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
