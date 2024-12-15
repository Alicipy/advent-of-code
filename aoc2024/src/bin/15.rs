use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

fn parse_input<R: BufRead>(reader: R) -> Result<(Vec<Vec<char>>, String)> {
    let mut labyrinth = vec![];
    let mut movement_lines = vec![];
    for l in reader.lines() {
        let l = l?;
        if l.starts_with('#') {
            labyrinth.push(l.chars().collect::<Vec<_>>());
        } else {
            movement_lines.push(l);
        }
    }
    let movement = movement_lines.join("");

    Ok((labyrinth, movement))
}

fn get_movement_dir(c: char) -> (i32, i32) {
    match c {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => panic!("at the disco"),
    }
}

fn search_start_pos(labyrinth: &[Vec<char>]) -> (usize, usize) {
    for (i, line) in labyrinth.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '@' {
                return (i, j);
            }
        }
    }
    panic!("at the disco again")
}

fn evaluate_result(labyrinth: &[Vec<char>]) -> u64 {
    let mut res = 0;
    for (i, line) in labyrinth.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                res += i * 100 + j;
            }
        }
    }
    res as u64
}

fn _make_mutable_movement(
    labyrinth: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    movement: (i32, i32),
) -> bool {
    let at = labyrinth[pos.0][pos.1];
    match at {
        '#' => false,
        '.' => true,
        'O' | '@' => {
            let new_pos = (
                (pos.0 as i32 + movement.0) as usize,
                (pos.1 as i32 + movement.1) as usize,
            );
            let res = _make_mutable_movement(labyrinth, new_pos, movement);
            match res {
                true => {
                    labyrinth[new_pos.0][new_pos.1] = at;
                    labyrinth[pos.0][pos.1] = '.';
                    true
                }
                false => false,
            }
        }
        _ => panic!("at the disco"),
    }
}

fn simulate(labyrinth: &[Vec<char>], c: char) -> Vec<Vec<char>> {
    let mut new_labyrinth = labyrinth.to_owned();
    let start_pos = search_start_pos(&new_labyrinth);
    let dir = get_movement_dir(c);

    _make_mutable_movement(&mut new_labyrinth, start_pos, dir);

    new_labyrinth
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let (labyrinth, movement) = parse_input(reader)?;

        let mut current_labyrinth = labyrinth;

        for m in movement.chars() {
            current_labyrinth = simulate(&current_labyrinth, m);
        }

        let result = evaluate_result(&current_labyrinth);

        Ok(result)
    }

    assert_eq!(10092, part1(BufReader::new(TEST1.as_bytes()))?);

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
