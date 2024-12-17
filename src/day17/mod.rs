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
            .map(|a| a as u128)
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

fn execute_program(program: &[u8], registers: &mut [u128]) -> Vec<u8> {
    let mut output = vec![];
    let mut ip = 0;
    loop {
        let code = &program[ip..];
        let opcode = code[0];
        let literal_operand = code[1];

        let combo_operand = match literal_operand {
            0..=3 => literal_operand as u128,
            4..=6 => registers[(literal_operand - 4) as usize],
            _ => 0, // invalid
        };

        match opcode {
            0 => {
                // adv: a div
                registers[0] =
                    (registers[0] as f64 / 2_u128.pow(combo_operand as u32) as f64) as u128;
            }
            1 => {
                // bxl: b xor literal
                registers[1] ^= literal_operand as u128;
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
                    (registers[0] as f64 / 2_u128.pow(combo_operand as u32) as f64) as u128;
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

impl SolutionGold<String, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let (_, program) = input.split_once("\n\n").unwrap();
        let program = program.split_once(": ").unwrap().1;
        let program = program
            .split(',')
            .map(fast_parse_int)
            .map(|a| a as u8)
            .collect::<Vec<_>>();

        // amount of lower bytes found
        find_recursive_gold(&program, 0, 0);

        // todo!("solution end")
        0
    }
}

fn find_recursive_gold(program: &[u8], index: usize, acc: u128) -> bool {
    if index == program.len() {
        println!("found: {}", acc);
        return true;
    }

    for cur_byte in 0u8..=255u8 {
        let i = acc + ((cur_byte as u128) << (index * 8));
        // println!("testing {i:02X} for depth {index}");
        let matching_bytes = execute_program_gold(program, &mut [i, 0, 0]);

        if matching_bytes == program.len() {
            println!("found: {}", i);
            return true;
        }

        if matching_bytes >= index + 1 {
            // found
            // println!("found byte: 0x{:02X} for depth {index}", i);
            if find_recursive_gold(program, index + 1, i) {
                return true;
            }
        }
    }

    return false;
}

fn execute_program_gold(program: &[u8], registers: &mut [u128]) -> usize {
    let mut output_i = 0;
    let mut ip = 0;
    loop {
        let code = &program[ip..];
        let opcode = code[0];
        let literal_operand = code[1];

        let combo_operand = match literal_operand {
            0..=3 => literal_operand as u128,
            4..=6 => registers[(literal_operand - 4) as usize],
            _ => 0, // invalid
        };

        match opcode {
            0 => {
                // adv: a div
                debug_assert_ne!(literal_operand, 7);
                registers[0] =
                    (registers[0] as f64 / 2_u128.pow(combo_operand as u32) as f64) as u128;
            }
            1 => {
                // bxl: b xor literal
                registers[1] ^= literal_operand as u128;
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
                    (registers[0] as f64 / 2_u128.pow(combo_operand as u32) as f64) as u128;
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
    assert_eq!(12345678, output);
}
