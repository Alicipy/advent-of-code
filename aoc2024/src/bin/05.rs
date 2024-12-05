use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn read_input<R: BufRead>(reader: R) -> (HashSet<String>, Vec<String>) {
    let mut rules = HashSet::new();
    let mut checks = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("|") {
            rules.insert(line);
        } else if line.contains(",") {
            checks.push(line);
        }
    }
    (rules, checks)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (rules, checks) = read_input(reader);

        let mut result = 0;

        for check in checks {
            let parts = check.split(",").collect::<Vec<&str>>();
            let ordered = parts.iter().tuple_combinations().all(|(&a, &b)| {
                let forbidden = format!("{:}|{:}", b, a);
                !rules.contains(&forbidden)
            });

            if ordered {
                result += parts[parts.len() / 2].parse::<usize>()?;
            }
        }

        Ok(result)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let (rules, checks) = read_input(reader);
        let rules = rules
            .iter()
            .map(|x| {
                let p = x.split_once("|").unwrap();
                (p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap())
            })
            .collect::<HashSet<(i32, i32)>>();

        let mut res = 0;
        for check in checks {
            let current_order = check
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let mut parts: HashSet<_> = HashSet::from_iter(current_order.clone());

            let mut order = vec![];
            while !parts.is_empty() {
                // Get number where no other number is in front (topological sort in slow)
                let a = *(parts
                    .iter()
                    .find(|x| parts.iter().all(|y| !rules.contains(&(*y, **x))))
                    .unwrap());
                parts.remove(&a);
                order.push(a);
            }
            if current_order != order {
                res += order[order.len() / 2];
            }
        }

        Ok(res)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
