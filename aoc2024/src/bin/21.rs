use crate::ActionField::{Enter, Failure, Movement, Num};
use crate::Moves::{Down, Left, Right, Up};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;
use std::vec;

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// Not my proudest achivement, but worked out.
// src/bin/21.rs:259 took 6986.517370348s.

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
fn split_input_parts(input: Vec<char>) -> Result<(u64, Vec<ActionField>), Error> {
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
    Ok((numeric_prefix, numeric_input))
}

struct KeyboardIterator<'a> {
    keyboard: Keyboard,
    from: Option<ActionField>,
    to: Option<ActionField>,
    current: VecDeque<ActionField>,
    prev_iterator: Box<dyn Iterator<Item = ActionField> + 'a>,
}

impl<'a> KeyboardIterator<'a> {
    fn new(
        keyboard: Keyboard,
        prev_iterator: Box<impl Iterator<Item = ActionField> + 'a>,
    ) -> KeyboardIterator<'a> {
        Self {
            keyboard,
            from: None,
            to: Some(Enter),
            current: VecDeque::new(),
            prev_iterator,
        }
    }
}

impl Iterator for KeyboardIterator<'_> {
    type Item = ActionField;

    fn next(&mut self) -> Option<Self::Item> {
        self.to?;

        let res = self.current.pop_front();
        match res {
            None => {
                self.from = self.to;
                let next = self.prev_iterator.next();
                self.to = next;
                match next {
                    None => None,
                    Some(_) => {
                        let moves = self
                            .keyboard
                            .find_optimal_ordering(self.from.unwrap(), self.to.unwrap());
                        self.current = moves.iter().map(|m| Movement(*m)).collect();
                        self.current.push_back(Enter);
                        self.next()
                    }
                }
            }
            Some(x) => Some(x),
        }
    }
}

fn find_minimal_length_input<'a>(
    wanted_result: Vec<ActionField>,
    num_keyboards: usize,
) -> KeyboardIterator<'a> {
    let keyboard_arrangement = vec![Keyboard::new_directional_keyboard(); num_keyboards];
    //keyboard_arrangement.push(Keyboard::new_numeric_keyboard());

    let mut current_iterator = KeyboardIterator::new(
        Keyboard::new_numeric_keyboard(),
        Box::new(wanted_result.into_iter()),
    );

    for keyboard in keyboard_arrangement.into_iter().rev() {
        let iterator = KeyboardIterator::new(keyboard, Box::new(current_iterator));
        current_iterator = iterator;
    }
    current_iterator
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn solve<R: BufRead>(reader: R, robots: usize) -> Result<u64> {
        let inputs = parse_input(reader);

        let result = inputs
            .into_par_iter()
            .map(|input| {
                let (numeric_prefix, numeric_input) = split_input_parts(input).unwrap();

                let minimal_length_input = find_minimal_length_input(numeric_input, robots);
                let minimal_length = minimal_length_input.count();

                numeric_prefix * minimal_length as u64
            })
            .sum();

        Ok(result)
    }

    assert_eq!(126384, solve(BufReader::new(TEST1.as_bytes()), 2)?);
    assert_eq!(1881090, solve(BufReader::new(TEST1.as_bytes()), 5)?);

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        solve(reader, 2)
    }

    assert_eq!(126384, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        solve(reader, 25)
    }

    assert_eq!(246810588779586, part2(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
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
