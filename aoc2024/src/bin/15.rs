use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

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

#[derive(Clone)]
struct LabObjectDefinition {
    id: u64,
    left_pos: Position,
    width: usize,
}

impl LabObjectDefinition {
    fn is_in(&self, pos: &Position) -> bool {
        pos.0 == self.left_pos.0 && self.left_pos.1 <= pos.1 && pos.1 < self.left_pos.1 + self.width
    }

    fn get_leftmost_position(&self) -> Position {
        self.left_pos
    }

    fn get_rightmost_position(&self) -> Position {
        (self.left_pos.0, self.left_pos.1 + self.width - 1)
    }

    fn get_all_positions(&self) -> Vec<Position> {
        (0..self.width)
            .map(|i| (self.left_pos.0, self.left_pos.1 + i))
            .collect()
    }
}

type LabBox = LabObjectDefinition;
type Wall = LabObjectDefinition;

type Position = (usize, usize);

fn reinterpret_labyrinth(lab: Vec<Vec<char>>, width: usize) -> (Position, Vec<LabBox>, Vec<Wall>) {
    let mut boxes = vec![];
    let mut walls = vec![];
    let mut start_pos = None;

    for (i, line) in lab.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            let resolved_pos = (i, width * j);
            match char {
                '@' => start_pos = Some(resolved_pos),
                'O' => boxes.push(LabBox {
                    id: boxes.len() as u64,
                    left_pos: resolved_pos,
                    width,
                }),
                '#' => walls.push(Wall {
                    id: boxes.len() as u64,
                    left_pos: resolved_pos,
                    width,
                }),
                '.' => {}
                _ => panic!("should not happen {char}"),
            }
        }
    }

    (start_pos.unwrap(), boxes, walls)
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

fn evaluate_boxes_result(boxes: &[LabBox]) -> u64 {
    let mut res = 0;

    for b in boxes {
        let pos = b.get_leftmost_position();
        res += pos.0 * 100 + pos.1;
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

fn change_position(pos: Position, movement: (i32, i32)) -> Position {
    (
        (pos.0 as i32 + movement.0) as usize,
        (pos.1 as i32 + movement.1) as usize,
    )
}

fn try_if_new_pos_allowed(
    pos: Position,
    boxes: &[LabBox],
    walls: &[Wall],
    movement: (i32, i32),
) -> Result<HashSet<u64>> {
    if walls.iter().any(|w| w.is_in(&pos)) {
        return Err(anyhow!("Nope, box"));
    }

    match boxes.iter().find(|b| b.is_in(&pos)) {
        None => Ok(HashSet::new()),
        Some(b) => {
            let mut potential: HashSet<_> = HashSet::new();
            potential.insert(b.id);
            let relevant_positions = match movement {
                (0, -1) => vec![b.get_leftmost_position()],
                (0, 1) => vec![b.get_rightmost_position()],
                _ => b.get_all_positions(),
            };
            for box_pos in relevant_positions {
                let new_pos = change_position(box_pos, movement);

                let res = try_if_new_pos_allowed(new_pos, boxes, walls, movement)?;
                for r in res {
                    potential.insert(r);
                }
            }
            Ok(potential)
        }
    }
}

fn apply_step(
    pos: Position,
    boxes: &[LabBox],
    walls: &[Wall],
    movement: (i32, i32),
) -> (Position, Vec<LabBox>) {
    let potential_pos = change_position(pos, movement);
    let potential_movement_result = try_if_new_pos_allowed(potential_pos, boxes, walls, movement);
    match potential_movement_result {
        Ok(blocks) => {
            let new_boxes = boxes
                .iter()
                .map(|b| {
                    if blocks.contains(&b.id) {
                        LabBox {
                            left_pos: change_position(b.left_pos, movement),
                            ..*b
                        }
                    } else {
                        b.clone()
                    }
                })
                .collect();
            (potential_pos, new_boxes)
        }
        Err(_) => (pos, boxes.to_vec()),
    }
}

fn solve<R: BufRead>(reader: R, width: usize) -> Result<u64> {
    let (labyrinth, movement) = parse_input(reader)?;

    let (mut pos, mut boxes, walls) = reinterpret_labyrinth(labyrinth, width);

    for c in movement.chars() {
        (pos, boxes) = apply_step(pos, &boxes, &walls, get_movement_dir(c));
    }

    let result = evaluate_boxes_result(&boxes);

    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        solve(reader, 1)
    }

    assert_eq!(10092, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        solve(reader, 2)
    }

    assert_eq!(9021, part2(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
