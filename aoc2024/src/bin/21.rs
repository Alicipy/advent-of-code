use crate::ActionField::{Enter, Failure, Movement, Num};
use crate::Moves::{Down, Left, Right, Up};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;
use std::vec;

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
029A
980A
179A
456A
379A
";

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum ActionField {
    Movement(Moves),
    Num(u8),
    Enter,
    Failure,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Moves {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Keyboard {
    keymap: Vec<Vec<ActionField>>,
}

impl Keyboard {
    fn new_directional_keyboard() -> Keyboard {
        Keyboard {
            keymap: vec![
                vec![Failure, Movement(Up), Enter],
                vec![Movement(Left), Movement(Down), Movement(Right)],
            ],
        }
    }
    fn new_numeric_keyboard() -> Keyboard {
        Keyboard {
            keymap: vec![
                vec![Num(7), Num(8), Num(9)],
                vec![Num(4), Num(5), Num(6)],
                vec![Num(1), Num(2), Num(3)],
                vec![Failure, Num(0), Enter], // 10 is a bit of a hack here, as 'A' is not an Enter here
            ],
        }
    }

    fn find_optimal_ordering(&self, from: ActionField, to: ActionField) -> Vec<Moves> {
        let from_position = self._find_position(from);
        let to_position = self._find_position(to);

        let mut res = vec![];

        if self.keymap.iter().len() == 4 {
            // from down rank to left side -> up and left
            if from_position.0 == 3 && to_position.1 == 0 {
                (0..(from_position.0 - to_position.0)).for_each(|_| res.push(Up));
                (0..(from_position.1 - to_position.1)).for_each(|_| res.push(Left));
            } else if from_position.1 == 0 && to_position.0 == 3 {
                (0..(to_position.1 - from_position.1)).for_each(|_| res.push(Right));
                (0..(to_position.0 - from_position.0)).for_each(|_| res.push(Down));
            } else {
                (0..from_position.1.saturating_sub(to_position.1)).for_each(|_| res.push(Left));
                (0..from_position.0.saturating_sub(to_position.0)).for_each(|_| res.push(Up));
                (0..to_position.0.saturating_sub(from_position.0)).for_each(|_| res.push(Down));
                (0..to_position.1.saturating_sub(from_position.1)).for_each(|_| res.push(Right));
            }
        } else if from_position.0 == 0 && to_position.1 == 0 {
            res.push(Down);
            (0..(from_position.1 - to_position.1)).for_each(|_| res.push(Left));
        } else if to_position.0 == 0 && from_position.1 == 0 {
            (0..(to_position.1 - from_position.1)).for_each(|_| res.push(Right));
            res.push(Up);
        } else {
            (0..from_position.1.saturating_sub(to_position.1)).for_each(|_| res.push(Left));
            (0..from_position.0.saturating_sub(to_position.0)).for_each(|_| res.push(Up));
            (0..to_position.0.saturating_sub(from_position.0)).for_each(|_| res.push(Down));
            (0..to_position.1.saturating_sub(from_position.1)).for_each(|_| res.push(Right));
        }
        res
    }

    fn _find_position(&self, elem: ActionField) -> (usize, usize) {
        let elem_position = self
            .keymap
            .iter()
            .flatten()
            .position(|k| k == &elem)
            .unwrap();
        
        (
            elem_position / self.keymap[0].len(),
            elem_position % self.keymap[0].len(),
        )
    }
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn find_minimal_length_input(
    keyboard_arrangement: Vec<Keyboard>,
    wanted_result: Vec<ActionField>,
) -> Vec<ActionField> {
    let mut current_state = wanted_result;

    for keyboard in keyboard_arrangement.into_iter().rev() {
        current_state.insert(0, Enter);
        //println!("{current_state:?}");

        let mut nextrow = vec![];

        for from_to in current_state.windows(2) {
            let from = from_to[0];
            let to = from_to[1];

            let optimal_paths = keyboard.find_optimal_ordering(from, to);

            for mv in optimal_paths {
                nextrow.push(Movement(mv))
            }
            nextrow.push(Enter);
        }
        current_state = nextrow;
    }
    //println!("{current_state:?}");
    current_state
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let inputs = parse_input(reader);

        let keyboard_setup = vec![
            Keyboard::new_directional_keyboard(),
            Keyboard::new_directional_keyboard(),
            Keyboard::new_numeric_keyboard(),
        ];

        let mut result = 0;

        for input in inputs.iter() {
            let numeric_prefix = String::from_iter(input[..=2].iter()).parse::<u64>()?;

            let numeric_input: Vec<_> = input[..=3]
                .iter()
                .map(|x| {
                    if *x == 'A' {
                        Enter
                    } else {
                        Num(x.to_digit(10).unwrap() as u8)
                    }
                })
                .collect();

            let minimal_length =
                find_minimal_length_input(keyboard_setup.clone(), numeric_input).len();

            result += numeric_prefix * minimal_length as u64;
        }

        Ok(result)
    }

    assert_eq!(126384, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    /*
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        Ok(result)
    }

    assert_eq!(0, part2(BufReader::new(TEST1.as_bytes()))?);
    */

    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_to_actions(s: String) -> Vec<ActionField> {
        s.chars()
            .map(|c| match c {
                'A' => Enter,
                'v' => Movement(Down),
                '^' => Movement(Up),
                '<' => Movement(Left),
                '>' => Movement(Right),
                _ => panic!("Wrong input string for test"),
            })
            .collect()
    }

    #[test]
    fn test_numeric_keyboard_optimal_pathing_upleft_edgecase() {
        let keyboard = Keyboard::new_numeric_keyboard();
        let res = keyboard.find_optimal_ordering(Num(0), Num(7));
        assert_eq!(res, vec![Up, Up, Up, Left])
    }

    #[test]
    fn test_numeric_keyboard_optimal_pathing_rightdown_edgecase() {
        let keyboard = Keyboard::new_numeric_keyboard();
        let res = keyboard.find_optimal_ordering(Num(7), Num(0));
        assert_eq!(res, vec![Right, Down, Down, Down]);
    }

    #[test]
    fn test_numeric_keyboard_big_field_3_7() {
        let keyboard = Keyboard::new_numeric_keyboard();
        let res = keyboard.find_optimal_ordering(Num(3), Num(7));
        assert_eq!(res, vec![Left, Left, Up, Up]);
    }

    #[test]
    fn test_numeric_keyboard_big_field_7_3() {
        let keyboard = Keyboard::new_numeric_keyboard();
        let res = keyboard.find_optimal_ordering(Num(7), Num(3));
        assert_eq!(res, vec![Down, Down, Right, Right]);
    }

    #[test]
    fn test_numeric_keyboard_big_field_9_1() {
        let keyboard = Keyboard::new_numeric_keyboard();
        let res = keyboard.find_optimal_ordering(Num(9), Num(1));
        assert_eq!(res, vec![Left, Left, Down, Down]);
    }

    #[test]
    fn test_numeric_keyboard_big_field_9_3() {
        let keyboard = Keyboard::new_numeric_keyboard();
        let res = keyboard.find_optimal_ordering(Num(9), Num(3));
        assert_eq!(res, vec![Down, Down]);
    }

    #[test]
    fn test_numeric_keyboard_big_field_0_2() {
        let keyboard = Keyboard::new_numeric_keyboard();
        let res = keyboard.find_optimal_ordering(Num(0), Num(2));
        assert_eq!(res, vec![Up]);
    }

    #[test]
    fn test_directional_enter_left_optimal_solved() {
        let keyboard = Keyboard::new_directional_keyboard();
        let res = keyboard.find_optimal_ordering(Enter, Movement(Left));
        assert_eq!(res, vec![Down, Left, Left]);
    }

    #[test]
    fn test_directional_left_enter_optimal_solved() {
        let keyboard = Keyboard::new_directional_keyboard();
        let res = keyboard.find_optimal_ordering(Movement(Left), Enter);
        assert_eq!(res, vec![Right, Right, Up]);
    }

    #[test]
    fn test_directional_enter_down_optimal_solved() {
        let keyboard = Keyboard::new_directional_keyboard();
        let res = keyboard.find_optimal_ordering(Enter, Movement(Down));
        assert_eq!(res, vec![Left, Down]);
    }

    #[test]
    fn test_directional_down_enter_optimal_solved() {
        let keyboard = Keyboard::new_directional_keyboard();
        let res = keyboard.find_optimal_ordering(Movement(Down), Enter);
        assert_eq!(res, vec![Up, Right]);
    }
}
