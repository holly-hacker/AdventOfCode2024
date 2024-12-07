use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<u64> for Day {
    const DAY: u32 = 7;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> u64 {
        input
            .lines()
            .map(|line| {
                let (out, rest) = line.split_once(':').unwrap();
                let expected_result = fast_parse_int(out) as u64;
                let rest = rest
                    .trim()
                    .split(" ")
                    .map(fast_parse_int)
                    .map(|a| a as u64)
                    .collect::<Vec<_>>();

                for i in 0..(1 << (rest.len())) {
                    let mut i = i;
                    let mut current_result = 0;
                    for num in &rest {
                        let operation = i & 1;
                        i >>= 1;

                        if operation == 0 {
                            current_result += num;
                        } else {
                            current_result *= num;
                        }
                    }

                    if current_result == expected_result {
                        return expected_result;
                    }
                }
                0
            })
            .sum()
    }
}

impl SolutionGold<u64, u64> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> u64 {
        input
            .lines()
            .map(|line| {
                let (out, rest) = line.split_once(':').unwrap();
                let expected_result = fast_parse_int(out) as u64;
                let rest = rest
                    .trim()
                    .split(" ")
                    .map(fast_parse_int)
                    .map(|a| a as u64)
                    .collect::<Vec<_>>();

                for i in 0..(3usize.pow(rest.len() as u32)) {
                    let mut i = i;
                    let mut current_result = 0u64;
                    for &num in &rest {
                        // if number is already too large, we won't get the expected result with future operations
                        if current_result >= expected_result {
                            break;
                        }

                        let operation = i % 3;
                        i /= 3;

                        match operation {
                            0 => current_result *= num,
                            1 => current_result += num,
                            // assume that the first number is never 0, as the exercise does not define it
                            2 if current_result == 0 => break,
                            2 => {
                                current_result = 10u64.pow(num.ilog10() + 1) * current_result + num;
                            }
                            _ => unreachable!(),
                        }
                    }

                    if current_result == expected_result {
                        return expected_result;
                    }
                }
                0
            })
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(3749, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(465126289353, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(11387, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(70597497486371, output);
}
