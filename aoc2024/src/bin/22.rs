use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "22";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
1
10
100
2024
";

struct SecretIterator {
    last_secret: u64,
}

impl Iterator for SecretIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur_secret = self.last_secret;

        // Calculate the result of multiplying the secret number by 64.
        // Then, mix this result into the secret number. Finally, prune the secret number.
        cur_secret = Self::prune(Self::mix(cur_secret, cur_secret * 64));

        // Calculate the result of dividing the secret number by 32.
        // Round the result down to the nearest integer.
        // Then, mix this result into the secret number. Finally, prune the secret number.
        cur_secret = Self::prune(Self::mix(cur_secret, cur_secret / 32));

        // Calculate the result of multiplying the secret number by 2048.
        // Then, mix this result into the secret number. Finally, prune the secret number.
        cur_secret = Self::prune(Self::mix(cur_secret, cur_secret * 2048));

        self.last_secret = cur_secret;

        Some(cur_secret)
    }
}

impl SecretIterator {
    fn mix(secret: u64, res: u64) -> u64 {
        secret ^ res
    }

    fn prune(secret: u64) -> u64 {
        secret % 16777216
    }
}

fn parse_input<R: BufRead>(reader: R) -> Vec<u64> {
    reader
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let numbers = parse_input(reader);

        let res = numbers
            .iter()
            .map(|s| SecretIterator { last_secret: *s })
            .map(|mut it| it.nth(1999).unwrap())
            .sum();

        Ok(res)
    }

    assert_eq!(37327623, part1(BufReader::new(TEST1.as_bytes()))?);

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
    fn iterator_next_123_first_10_correct() {
        let iter = SecretIterator { last_secret: 123 };

        assert_eq!(
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ],
            iter.take(10).collect::<Vec<u64>>()
        );
    }
}
