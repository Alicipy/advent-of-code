use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::{max, min, Ordering};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn parse_input<R: BufRead>(reader: R) -> HashSet<(String, String)> {
    reader
        .lines()
        .map(|line| {
            let binding = line.unwrap();
            let (a, b) = binding.split_once('-').unwrap();
            let a = a.to_string();
            let b = b.to_string();
            (min(a.clone(), b.clone()), max(a, b))
        })
        .collect()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let connections = parse_input(reader);

        let mut known_computers = HashSet::new();
        for (c1, c2) in connections.iter() {
            known_computers.insert(c1.to_owned());
            known_computers.insert(c2.to_owned());
        }

        let mut known_with_t = 0;

        for a in known_computers.iter() {
            for b in known_computers
                .iter()
                .filter(|d| a.cmp(d) == Ordering::Less)
            {
                for c in known_computers
                    .iter()
                    .filter(|d| b.cmp(d) == Ordering::Less)
                {
                    if connections.contains(&(a.clone(), b.clone()))
                        && connections.contains(&(a.clone(), c.clone()))
                        && connections.contains(&(b.clone(), c.clone()))
                        && (a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
                    {
                        known_with_t += 1;
                    }
                }
            }
        }

        Ok(known_with_t)
    }

    assert_eq!(7, part1(BufReader::new(TEST1.as_bytes()))?);

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
