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

fn read_input<R: BufRead>(mut reader: R) -> Vec<u8> {
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer);
    let input = buffer
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect_vec();
    input
}

pub struct UncompressIterator<'a> {
    v: &'a Vec<u8>,
    idx: i32,
    cur_val: u8,
    max_val: u8,
}

fn get_uncompress_iterator(vec: &Vec<u8>) -> UncompressIterator {
    UncompressIterator {
        v: vec,
        idx: -1,
        cur_val: 0,
        max_val: 0,
    }
}

impl<'a> Iterator for UncompressIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_val < self.max_val {
            self.cur_val += 1;
            Some(self.idx)
        } else {
            self.cur_val = 0;
            self.idx += 1;
            if self.idx >= self.v.len() as i32 {
                return None;
            }
            self.max_val = self.v[self.idx as usize];
            self.next()
        }
    }
}

struct Hole {
    position: u32,
    size: u32,
}

struct BlockFile {
    start_pos: u32,
    size: u32,
    idx: u32,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let input = read_input(reader);
        let front: Vec<_> = input.clone().into_iter().step_by(2).collect();
        let back: Vec<_> = input.clone().into_iter().rev().step_by(2).collect();

        let num_blocks = front.iter().fold(0i32, |acc, e| acc + (*e as i32));
        let max_block_id = (back.len() - 1) as i32;

        let mut front = get_uncompress_iterator(&front);
        let mut back = get_uncompress_iterator(&back);
        let mut uncompressed = get_uncompress_iterator(&input);

        let mut result = 0i64;
        for i in 0..num_blocks {
            let num = uncompressed.next().unwrap();
            let block_id = if num % 2 == 0 {
                front.next().unwrap()
            } else {
                max_block_id - back.next().unwrap()
            };
            result += (i * block_id) as i64;
        }

        Ok(result)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let input = read_input(reader);

        let mut files = vec![];
        let mut holes = vec![];
        let mut pos = 0;
        for (i, val) in input.into_iter().enumerate() {
            let val = val as u32;
            if i % 2 == 0 {
                files.push(BlockFile {
                    idx: i as u32 / 2,
                    start_pos: pos,
                    size: val,
                });
            } else {
                holes.push(Hole {
                    position: pos,
                    size: val,
                });
            }
            pos += val;
        }

        for blockfile in files.iter_mut().rev() {
            let first_hole = holes
                .iter_mut()
                .find(|h| h.size >= blockfile.size && h.position < blockfile.start_pos);
            if let Some(hole) = first_hole {
                blockfile.start_pos = hole.position;
                hole.size -= blockfile.size;
                hole.position += blockfile.size;
            };
        }

        let mut result = 0;
        for block_file in files.iter() {
            for i in 0..block_file.size {
                result += ((block_file.start_pos + i) * block_file.idx) as u64;
            }
        }

        Ok(result)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
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
