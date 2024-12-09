use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

pub struct UncompressIterator<'a> {
    v: &'a Vec<i8>,
    idx: i64,
    cur_val: i8,
    max_val: i8,
}

fn get_uncompress_iterator(vec: &Vec<i8>) -> UncompressIterator {
    UncompressIterator {
        v: vec,
        idx: -1,
        cur_val: -1,
        max_val: -1,
    }
}

impl<'a> Iterator for UncompressIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_val < self.max_val {
            self.cur_val += 1;
            Some(self.idx)
        } else {
            self.cur_val = 0;
            self.idx += 1;
            if self.idx >= self.v.len() as i64 {
                return None;
            }
            self.max_val = self.v[self.idx as usize];
            self.next()
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<i64> {
        let mut buffer = String::new();
        let _ = reader.read_line(&mut buffer);
        let input = buffer
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .collect_vec();
        let front: Vec<_> = input.clone().into_iter().step_by(2).collect();
        let back: Vec<_> = input.clone().into_iter().rev().step_by(2).collect();

        let num_blocks = front.iter().fold(0i64, |acc, e| acc + (*e as i64));
        let max_block_id = (back.len() - 1) as i64;

        let mut front = get_uncompress_iterator(&front);
        let mut back = get_uncompress_iterator(&back);
        let mut uncompressed = get_uncompress_iterator(&input);

        let mut result = 0;
        for i in 0..num_blocks {
            let num = uncompressed.next().unwrap();
            let block_id = if num % 2 == 0 {
                front.next().unwrap()
            } else {
                max_block_id - back.next().unwrap()
            };
            result += i * block_id;
        }

        Ok(result)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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
    fn test_simple() {
        let input = vec![1, 2, 3];
        let iter = get_uncompress_iterator(&input);
        let result = iter.collect::<Vec<_>>();
        assert_eq!(vec![0, 1, 1, 2, 2, 2], result);
    }

    #[test]
    fn test_with_zero_at_start() {
        let input = vec![0, 2, 1, 2, 0];
        let iter = get_uncompress_iterator(&input);
        let result = iter.collect::<Vec<_>>();
        assert_eq!(vec![1, 1, 2, 3, 3], result);
    }
}
