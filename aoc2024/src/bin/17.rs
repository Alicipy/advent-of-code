use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

#[derive(Debug)]
struct ProgramState {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pc: usize,
}

fn parse_input<R: BufRead>(reader: R) -> Result<(ProgramState, Vec<u8>)> {
    let mut relevant_input_parts = vec![];
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let part = line.split_once(": ").unwrap().1.to_string();
        relevant_input_parts.push(part);
    }
    assert_eq!(relevant_input_parts.len(), 4);

    let reg_a = relevant_input_parts[0].parse::<u64>()?;
    let reg_b = relevant_input_parts[1].parse::<u64>()?;
    let reg_c = relevant_input_parts[2].parse::<u64>()?;

    let instructions = relevant_input_parts[3]
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let state = ProgramState {
        reg_a,
        reg_b,
        reg_c,
        pc: 0,
    };

    Ok((state, instructions))
}

fn get_combo_operand_value(combo_operand: u8, program_state: &ProgramState) -> u64 {
    match combo_operand {
        0..=3 => combo_operand as u64,
        4 => program_state.reg_a,
        5 => program_state.reg_b,
        6 => program_state.reg_c,
        _ => panic!("invalid combo operand '{}'", combo_operand),
    }
}

fn simulate(state: ProgramState, instructions: &[u8]) -> Option<(ProgramState, Option<u8>)> {
    let pc = state.pc;
    if pc >= instructions.len() {
        return None;
    }
    let opcode = instructions[pc];
    let operand = instructions[pc + 1];

    let mut output = None;

    let new_state = match opcode {
        0 => {
            let numerator = state.reg_a;
            let div_base_val = get_combo_operand_value(operand, &state);
            let div_res = numerator / (2_u64.pow(div_base_val as u32));

            ProgramState {
                reg_a: div_res,
                pc: pc + 2,
                ..state
            }
        }
        1 => {
            let new_reg_b = state.reg_b ^ (operand as u64);
            ProgramState {
                reg_b: new_reg_b,
                pc: pc + 2,
                ..state
            }
        }
        2 => {
            let mod_val = get_combo_operand_value(operand, &state) % 8;
            ProgramState {
                reg_b: mod_val,
                pc: pc + 2,
                ..state
            }
        }
        3 => ProgramState {
            pc: if state.reg_a == 0 {
                pc + 2
            } else {
                operand as usize
            },
            ..state
        },
        4 => ProgramState {
            reg_b: state.reg_b ^ state.reg_c,
            pc: pc + 2,
            ..state
        },
        5 => {
            let val = get_combo_operand_value(operand, &state) % 8;
            output = Some(val as u8);

            ProgramState {
                pc: pc + 2,
                ..state
            }
        }
        6 => {
            let numerator = state.reg_a;
            let div_base_val = get_combo_operand_value(operand, &state);
            let div_res = numerator / (2_u64.pow(div_base_val as u32));

            ProgramState {
                reg_b: div_res,
                pc: pc + 2,
                ..state
            }
        }
        7 => {
            let numerator = state.reg_a;
            let div_base_val = get_combo_operand_value(operand, &state);
            let div_res = numerator / (2_u64.pow(div_base_val as u32));

            ProgramState {
                reg_c: div_res,
                pc: pc + 2,
                ..state
            }
        }
        _ => {
            panic!("invalid opcode '{}'", opcode);
        }
    };

    Some((new_state, output))
}

fn simulate_program(initial_program_state: ProgramState, instructions: &[u8]) -> Vec<u8> {
    let mut current_program_state = initial_program_state;

    let mut full_output = vec![];

    while let Some((state, output)) = simulate(current_program_state, instructions) {
        current_program_state = state;
        if let Some(content) = output {
            full_output.push(content);
        }
    }
    full_output
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        let (initial_program_state, instructions) = parse_input(reader)?;

        let full_output = simulate_program(initial_program_state, &instructions);

        Ok(full_output
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(","))
    }

    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST1.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn custom_part2_step(a: u64) -> Vec<u8> {
        let mut a = a;
        let mut output = vec![];

        while a > 0 {
            // 3,0
            let mut b = a % 8; // 2,4
            b ^= 3; // 1,3
            let c = a >> b; // 7,5
            a >>= 3; // 0,3
            b ^= c; // 4,3
            b ^= 5; // 1,5
            output.push((b % 8) as u8); // 5,5
                                        // 3,0
        }

        output
    }

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let (initial_program_state, instructions) = parse_input(reader)?;

        let mut cur_result = 0_u64;

        for i in 0..instructions.len() {
            let expected_result: Vec<_> =
                instructions.clone()[(instructions.len() - i - 1)..].to_vec();

            println!("{expected_result:?}");
            for j in 0.. {
                let new_reg_a = (cur_result << 3) + j;
                let full_output = custom_part2_step(new_reg_a);
                if full_output == expected_result {
                    cur_result = new_reg_a;
                    break;
                }
            }
        }

        let check = simulate_program(
            ProgramState {
                reg_a: cur_result,
                ..initial_program_state
            },
            &instructions,
        );

        assert_eq!(instructions, check);

        Ok(cur_result)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
