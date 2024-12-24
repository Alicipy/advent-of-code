use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "24";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

#[derive(Eq, PartialEq, Hash, Clone)]
struct Var {
    name: String,
}

struct BoolExpr {
    left: Var,
    right: Var,
    target: Var,
    expr: fn(u8, u8) -> u8,
}

impl BoolExpr {
    fn evaluate(&self, known_vals: &mut HashMap<Var, u8>) {
        let sources = (known_vals.get(&self.left), known_vals.get(&self.right));
        if let (Some(x), Some(y)) = sources {
            let res = (self.expr)(*x, *y);
            known_vals.insert(self.target.clone(), res);
        };
    }
}

fn parse_input<R: BufRead>(reader: R) -> Result<(Vec<BoolExpr>, HashMap<Var, u8>)> {
    let mut vars = vec![];
    let mut known_vals = HashMap::new();

    let var_regex = regex::Regex::new(r"^(\w+): ([01])$")?;
    let calc_regex = regex::Regex::new(r"^(\w+) (.+) (\w+) -> (\w+)$")?;

    for line in reader.lines() {
        let line = line?;

        if let Some(capt) = var_regex.captures(&line) {
            let name = capt[1].to_string();
            let num = capt[2].parse::<u8>()?;

            known_vals.insert(Var { name }, num);
        } else if let Some(capt) = calc_regex.captures(&line) {
            let left = Var {
                name: capt[1].to_string(),
            };
            let right = Var {
                name: capt[3].to_string(),
            };
            let target = Var {
                name: capt[4].to_string(),
            };

            let op = match &capt[2] {
                "AND" => |a, b| a & b,
                "OR" => |a, b| a | b,
                "XOR" => |a, b| a ^ b,
                _ => unreachable!(),
            };

            vars.push(BoolExpr {
                left,
                right,
                target,
                expr: op,
            });
        }
    }

    Ok((vars, known_vals))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let (vars, mut known_vals) = parse_input(reader)?;

        loop {
            let prev_size = known_vals.len();
            for var in vars.iter() {
                var.evaluate(&mut known_vals);
            }

            if known_vals.len() == prev_size {
                break;
            }
        }

        Ok(u64::from_str_radix(
            &known_vals
                .into_iter()
                .filter(|(name, _)| name.name.starts_with('z'))
                .sorted_by(|(a, _), (b, _)| {
                    b.name[1..]
                        .parse::<u8>()
                        .unwrap()
                        .cmp(&a.name[1..].parse::<u8>().unwrap())
                })
                .map(|(_, val)| val)
                .join(""),
            2,
        )?)
    }

    assert_eq!(2024, part1(BufReader::new(TEST1.as_bytes()))?);

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
