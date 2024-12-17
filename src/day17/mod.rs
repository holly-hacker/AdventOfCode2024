use std::collections::HashSet;

use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<String> for Day {
    const DAY: u32 = 17;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> String {
        let (registers, program) = input.split_once("\n\n").unwrap();
        let mut registers = registers
            .lines()
            .map(|line| line.split_once(": ").unwrap().1)
            .map(fast_parse_int)
            .map(|a| a as u64)
            .collect::<Vec<_>>();
        let program = program.split_once(": ").unwrap().1;
        let program = program
            .split(',')
            .map(fast_parse_int)
            .map(|a| a as u8)
            .collect::<Vec<_>>();

        execute_program(&program, &mut registers)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn execute_program(program: &[u8], registers: &mut [u64]) -> Vec<u8> {
    let mut output = vec![];
    let mut ip = 0;
    loop {
        let code = &program[ip..];
        let opcode = code[0];
        let literal_operand = code[1];

        let combo_operand = match literal_operand {
            0..=3 => literal_operand as u64,
            4..=6 => registers[(literal_operand - 4) as usize],
            _ => 0, // invalid
        };

        match opcode {
            0 => {
                // adv: a div
                registers[0] =
                    (registers[0] as f64 / 2_u64.pow(combo_operand as u32) as f64) as u64;
            }
            1 => {
                // bxl: b xor literal
                registers[1] ^= literal_operand as u64;
            }
            2 => {
                // bst: b store
                registers[1] = combo_operand % 8;
            }
            3 => {
                // jnz, jump not zero
                if registers[0] != 0 {
                    ip = literal_operand as usize;
                    continue;
                }
            }
            4 => {
                // bxc, B xor C
                registers[1] ^= registers[2];
            }
            5 => {
                // out
                output.push((combo_operand % 8) as u8);
            }
            6 => todo!("6"),
            7 => {
                // cdv, c div
                registers[2] =
                    (registers[0] as f64 / 2_u64.pow(combo_operand as u32) as f64) as u64;
            }
            _ => unreachable!("unknown opcode {opcode}"),
        }

        ip += 2;

        if ip >= program.len() {
            break;
        }
    }

    output
}

impl SolutionGold<String, u64> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> u64 {
        let (_, program) = input.split_once("\n\n").unwrap();
        let program = program.split_once(": ").unwrap().1;
        let program = program
            .split(',')
            .map(fast_parse_int)
            .map(|a| a as u8)
            .collect::<Vec<_>>();

        /*
        Example disassembly:
        A = input()
        loop:
            var1 = A % 8              # take lower 3 bits, values 0-7
            var2 = var1 ^ 1           # not important
            var3 = A / (2**var2)      # remove between 0 and 7 bits
            write(var2 ^ 0b101 ^ var3)# write lower 3 bits
            A /= 8                    # drop lower 3 bits
            if A == 0: exit()
         */
        let mut found = None;
        find_recursive_gold(&program, 0, 0, &mut found);
        found.unwrap()
    }
}

fn find_recursive_gold(program: &[u8], depth: usize, acc: u64, best_found: &mut Option<u64>) {
    if depth == program.len() {
        let prev_best = best_found.unwrap_or(u64::MAX);
        let cur_best = acc;
        let new_best = prev_best.min(cur_best);
        if cur_best < prev_best {
            *best_found = Some(new_best);
        }

        return;
    }

    let mut found = HashSet::new();
    for cur_val in 0..(1 << (7 + 3)) {
        debug_assert!(acc < (1 << (depth * 3)));
        let i = acc + ((cur_val as u64) << (depth * 3));

        // if we already found a better solution, exit
        if let Some(best_found) = best_found {
            if i >= *best_found {
                continue;
            }
        }

        let matching_byte_count = execute_program_gold(program, &mut [i, 0, 0]);

        // if no match, continue loop with next num
        if matching_byte_count <= depth {
            continue;
        }

        // found
        let masked_i = i & ((1 << ((depth + 1) * 3)) - 1);

        if let Some(best_found) = best_found {
            if masked_i > *best_found {
                continue;
            }
        }
        found.insert(masked_i);
    }
    for masked_i in found {
        find_recursive_gold(program, depth + 1, masked_i, best_found);
    }
}

fn execute_program_gold(program: &[u8], registers: &mut [u64]) -> usize {
    let mut output_i = 0;
    let mut ip = 0;
    loop {
        let code = &program[ip..];
        let opcode = code[0];
        let literal_operand = code[1];

        let combo_operand = match literal_operand {
            0..=3 => literal_operand as u64,
            4..=6 => registers[(literal_operand - 4) as usize],
            _ => 0, // invalid
        };

        match opcode {
            0 => {
                // adv: a div
                debug_assert_ne!(literal_operand, 7);
                registers[0] =
                    (registers[0] as f64 / 2_u64.pow(combo_operand as u32) as f64) as u64;
            }
            1 => {
                // bxl: b xor literal
                registers[1] ^= literal_operand as u64;
            }
            2 => {
                // bst: b store
                debug_assert_ne!(literal_operand, 7);
                registers[1] = combo_operand % 8;
            }
            3 => {
                // jnz, jump not zero
                if registers[0] != 0 {
                    ip = literal_operand as usize;
                    continue;
                }
            }
            4 => {
                // bxc, B = B xor C, ignores operand
                registers[1] ^= registers[2];
            }
            5 => {
                // out
                debug_assert_ne!(literal_operand, 7);
                let val = (combo_operand % 8) as u8;

                if output_i >= program.len() {
                    return output_i;
                }

                if program[output_i] != val {
                    return output_i;
                }
                output_i += 1;
            }
            6 => todo!("6"),
            7 => {
                // cdv, c div
                debug_assert_ne!(literal_operand, 7);
                registers[2] =
                    (registers[0] as f64 / 2_u64.pow(combo_operand as u32) as f64) as u64;
            }
            _ => unreachable!("unknown opcode {opcode}"),
        }

        ip += 2;

        if ip >= program.len() {
            break;
        }
    }

    output_i
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!("4,6,3,5,6,3,5,2,1,0".to_string(), output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!("6,4,6,0,4,5,7,2,7".to_string(), output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(117440, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(164541160582845, output);
}
