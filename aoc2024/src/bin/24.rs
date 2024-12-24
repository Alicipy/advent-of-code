use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::cmp::{max_by_key, min_by_key};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::ToString;

// Also not my proudest achivement, but worked out.
// src/bin/24.rs:459 took 168.267779716s.

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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Var {
    name: String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
enum BinOp {
    AND,
    OR,
    XOR,
}

impl BinOp {
    fn call(&self, left: u8, right: u8) -> u8 {
        match self {
            BinOp::AND => left & right,
            BinOp::OR => left | right,
            BinOp::XOR => left ^ right,
        }
    }
}

#[derive(Debug, Clone)]
struct BoolExpr {
    left: Var,
    right: Var,
    target: Var,
    op: BinOp,
}

impl BoolExpr {
    fn evaluate(&self, known_vals: &mut HashMap<Var, u8>) {
        let sources = (known_vals.get(&self.left), known_vals.get(&self.right));
        if let (Some(x), Some(y)) = sources {
            let res = (self.op).call(*x, *y);
            known_vals.insert(self.target.clone(), res);
        };
    }
}

#[derive(Debug)]
struct ExprSearchPattern {
    left: Option<Var>,
    right: Option<Var>,
    target: Option<Var>,
    op: BinOp,
}

impl ExprSearchPattern {
    fn search_for_expression<'a>(&self, exprs: &'a [BoolExpr]) -> Option<&'a BoolExpr> {
        let possib_res: Vec<_> = exprs
            .iter()
            .filter(|&e| {
                (self.left.is_none()
                    || self.left.as_ref().unwrap() == &e.left
                    || self.left.as_ref().unwrap() == &e.right)
                    && (self.right.is_none()
                        || self.right.as_ref().unwrap() == &e.right
                        || self.right.as_ref().unwrap() == &e.left)
                    && (self.target.is_none() || self.target.as_ref().unwrap() == &e.target)
                    && (self.op == e.op)
            })
            .collect();

        if possib_res.is_empty() {
            None
        } else if possib_res.len() == 1 {
            Some(possib_res[0])
        } else {
            panic!();
        }
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
            let left_orig = Var {
                name: capt[1].to_string(),
            };
            let right_orig = Var {
                name: capt[3].to_string(),
            };
            let target = Var {
                name: capt[4].to_string(),
            };

            let left = min_by_key(left_orig.clone(), right_orig.clone(), |x| x.name.clone());
            let right = max_by_key(left_orig, right_orig, |x| x.name.clone());

            let op = match &capt[2] {
                "AND" => BinOp::AND,
                "OR" => BinOp::OR,
                "XOR" => BinOp::XOR,
                _ => unreachable!(),
            };

            vars.push(BoolExpr {
                left,
                right,
                target,
                op,
            });
        }
    }

    Ok((vars, known_vals))
}

fn construct_set_partition_of_two<T: std::clone::Clone>(vars: &Vec<T>) -> Vec<Vec<(T, T)>> {
    if vars.len() == 2 {
        return vec![vec![(vars[0].clone(), vars[1].clone())]];
    }
    let mut vars = vars.to_owned();
    let last = vars.pop().unwrap();

    let mut result = vec![];

    for i in 0..vars.len() {
        let mut vars = vars.clone();
        let other = vars.remove(i);

        let current_set_part = (last.clone(), other);

        let set_partition_of_rest = construct_set_partition_of_two(&vars);
        for sp in set_partition_of_rest {
            let mut sp = sp;
            sp.push(current_set_part.clone());
            result.push(sp);
        }
    }
    result
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let (vars, known_vals) = parse_input(reader)?;
        let resolved = resolve(&vars, known_vals);
        Ok(serialize(resolved)?)
    }
    fn resolve(vars: &[BoolExpr], known_vals: HashMap<Var, u8>) -> HashMap<Var, u8> {
        let mut vals = known_vals;
        loop {
            let prev_size = vals.len();
            for var in vars.iter() {
                var.evaluate(&mut vals);
            }

            if vals.len() == prev_size {
                break;
            }
        }
        vals
    }
    fn serialize(vals: HashMap<Var, u8>) -> Result<u64> {
        Ok(u64::from_str_radix(
            &vals
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let (exprs, _) = parse_input(reader)?;

        let (safe_wrong, maybe_wrong) = search_anomalies(&exprs);

        let result: Vec<Option<Vec<(Var, Var)>>> = maybe_wrong
            .into_iter()
            .combinations(8 - safe_wrong.len())
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|maybe| {
                let mut selected_set = safe_wrong.clone();
                selected_set.extend(maybe);
                assert_eq!(selected_set.len(), 8);

                let exprs = exprs.clone();

                for permutation in construct_set_partition_of_two(&selected_set) {
                    let mut switches = HashMap::new();
                    for (a, b) in permutation.clone() {
                        switches.insert(a.clone(), b.clone());
                        switches.insert(b.clone(), a.clone());
                    }
                    let exprs: Vec<_> = exprs
                        .iter()
                        .map(|e| {
                            if switches.contains_key(&e.target) {
                                let res = BoolExpr {
                                    target: switches.get(&e.target).unwrap().clone(),
                                    ..e.clone()
                                };
                                res
                            } else {
                                e.clone()
                            }
                        })
                        .collect();

                    let feasible = check_add_correctness(&exprs);
                    if feasible {
                        return Some(permutation);
                    }
                }
                None
            })
            .filter(|x| !x.is_none())
            .collect();

        assert_eq!(result.len(), 1);
        Ok(result[0]
            .clone()
            .unwrap()
            .into_iter()
            .flat_map(|x| vec![x.0.name, x.1.name])
            .sorted()
            .join(","))
    }

    fn check_add_correctness(expressions: &[BoolExpr]) -> bool {
        for _i in 0..200 {
            let x = rand::random_range(0..(2_u64.pow(45)));
            let y = rand::random_range(0..(2_u64.pow(45)));
            let expected_z = x + y;

            let mut known_vals = HashMap::new();
            for i in 0..45 {
                known_vals.insert(
                    Var {
                        name: format!("x{:0>2}", i),
                    },
                    ((x >> i) & 1) as u8,
                );
                known_vals.insert(
                    Var {
                        name: format!("y{:0>2}", i),
                    },
                    ((y >> i) & 1) as u8,
                );
            }

            let resolved_vars = resolve(expressions, known_vals);
            let resolved_vars: HashMap<_, _> = resolved_vars
                .into_iter()
                .filter(|x| x.0.name.starts_with("z"))
                .collect();

            let num_z = resolved_vars.len();

            if num_z < 46 {
                return false;
            }

            let calculated_z = serialize(resolved_vars).unwrap();

            if expected_z != calculated_z {
                return false;
            }
        }

        true
    }

    // check for wrong wires based on
    // https://de.wikipedia.org/wiki/Volladdierer#/media/Datei:Volladdierer_Aufbau_DIN40900.svg
    fn search_anomalies(exprs: &Vec<BoolExpr>) -> (Vec<Var>, Vec<Var>) {
        let mut safe_wrong = vec![];
        let mut maybe_wrong = vec![];

        for i in 1..45 {
            let x = Some(Var {
                name: format!("x{:0>2}", i),
            });
            let y = Some(Var {
                name: format!("y{:0>2}", i),
            });
            let z = Var {
                name: format!("z{:0>2}", i),
            };

            let ha_1_and = ExprSearchPattern {
                left: x.clone(),
                right: y.clone(),
                op: BinOp::AND,
                target: None,
            }
            .search_for_expression(exprs)
            .unwrap();
            let ha_1_xor = ExprSearchPattern {
                left: x.clone(),
                right: y.clone(),
                op: BinOp::XOR,
                target: None,
            }
            .search_for_expression(exprs)
            .unwrap();

            let ha_1_and_part_of_or = ExprSearchPattern {
                left: Some(ha_1_and.target.clone()),
                right: None,
                op: BinOp::OR,
                target: None,
            }
            .search_for_expression(exprs);
            if ha_1_and_part_of_or.is_none() {
                safe_wrong.push(ha_1_and.target.clone());
                continue;
            }

            let ha_2_and = ExprSearchPattern {
                left: Some(ha_1_xor.target.clone()),
                right: None,
                op: BinOp::AND,
                target: None,
            }
            .search_for_expression(exprs)
            .unwrap();
            let ha_2_xor = ExprSearchPattern {
                left: Some(ha_1_xor.target.clone()),
                right: None,
                op: BinOp::XOR,
                target: None,
            }
            .search_for_expression(exprs)
            .unwrap();

            if ha_2_xor.target.clone() != z {
                safe_wrong.push(ha_2_xor.target.clone());
                safe_wrong.push(z);
                continue;
            }

            ExprSearchPattern {
                left: Some(ha_1_and.target.clone()),
                right: Some(ha_2_and.target.clone()),
                op: BinOp::OR,
                target: None,
            }
            .search_for_expression(exprs)
            .unwrap();
        }

        for e in exprs {
            let v = Var { ..e.target.clone() };
            if !safe_wrong.contains(&v) {
                maybe_wrong.push(v);
            }
        }

        (safe_wrong, maybe_wrong)
    }

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
    fn test_construct_set_partition_of_two_four_values() {
        let test_value = vec![1, 2, 3, 4];
        let partition = construct_set_partition_of_two(&test_value);
        assert_eq!(partition.len(), 3);
    }
    #[test]
    fn test_construct_set_partition_of_two_six_values() {
        let test_value = vec![1, 2, 3, 4, 5, 6];
        let partition = construct_set_partition_of_two(&test_value);
        assert_eq!(partition.len(), 15);
    }
}
