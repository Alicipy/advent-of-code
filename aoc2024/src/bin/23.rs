use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Node(String);

#[derive(Eq, Hash, PartialEq)]
struct Connection {
    n1: Node,
    n2: Node,
}

impl Connection {
    fn new(n1: Node, n2: Node) -> Self {
        Connection {
            n1: min(n1.clone(), n2.clone()),
            n2: max(n1, n2),
        }
    }
}

fn parse_input<R: BufRead>(reader: R) -> HashSet<Connection> {
    reader
        .lines()
        .map(|line| {
            let binding = line.unwrap();
            let (a, b) = binding.split_once('-').unwrap();
            let a = a.to_string();
            let b = b.to_string();
            Connection::new(Node(a), Node(b))
        })
        .collect()
}

fn construct_cliques(connections: &HashSet<Connection>) -> Vec<HashSet<Node>> {
    let all_nodes: HashSet<_> = connections
        .iter()
        .flat_map(|f| vec![f.n1.clone(), f.n2.clone()])
        .collect();

    let mut all_cliques: Vec<HashSet<_>> = vec![];

    let mut last_step_cliques = vec![HashSet::new()];

    while !last_step_cliques.is_empty() {
        all_cliques.extend(last_step_cliques.clone());

        let mut new_cliques = vec![];

        for c in last_step_cliques.into_iter() {
            for node in all_nodes.iter() {
                if c.clone()
                    .into_iter()
                    .all(|cn| connections.contains(&Connection::new(node.clone(), cn)))
                {
                    let mut new_clique = c.clone();
                    new_clique.insert(node.clone());
                    new_cliques.push(new_clique);
                }
            }
        }

        new_cliques.sort_by(|a, b| {
            let av: Vec<Node> = a.clone().into_iter().sorted().collect();
            let ab: Vec<Node> = b.clone().into_iter().sorted().collect();

            av.cmp(&ab)
        });
        new_cliques.dedup();

        last_step_cliques = new_cliques;
        println!("{:?}", last_step_cliques.len());
    }

    all_cliques
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let connections = parse_input(reader);

        let all_cliques = construct_cliques(&connections);

        let known_with_t = all_cliques
            .iter()
            .filter(|c| c.iter().count() == 3)
            .filter(|s| s.iter().any(|n| n.0.starts_with("t")))
            .count() as u64;

        Ok(known_with_t)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    /*
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    */
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let connections = parse_input(reader);

        let all_cliques = construct_cliques(&connections);

        let biggest_clique = all_cliques.last().unwrap();

        let mut nodes = biggest_clique.iter().collect_vec();
        nodes.sort();
        let output = nodes.into_iter().map(|n| n.0.to_string()).join(",");

        Ok(output)
    }

    assert_eq!(
        "co,de,ka,ta".to_string(),
        part2(BufReader::new(TEST.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion
    //

    Ok(())
}
