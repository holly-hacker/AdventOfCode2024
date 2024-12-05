use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 5;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();

        // all numbers seem to be from 11 to 99 (inclusive)
        // this fits in a 128bit bitmap
        const BUFFER_LEN: usize = 99 - 11 + 1;
        let mut rules = [0u128; BUFFER_LEN];

        let mut index = 0;
        loop {
            let num1_1 = input[index];
            if num1_1 == b'\n' {
                break;
            }

            let num1_1 = num1_1 - b'0';
            let num1_2 = input[index + 1] - b'0';
            let num2_1 = input[index + 3] - b'0';
            let num2_2 = input[index + 4] - b'0';

            let num1 = (num1_1 * 10 + num1_2) as usize;
            let num2 = (num2_1 * 10 + num2_2) as usize;

            rules[num1 - 11] |= 1 << num2;
            index += 6;
        }

        // skip empty line
        index += 1;

        let mut sum = 0;
        while index < input.len() {
            let mut seen = 0u128;
            let start_index = index;
            let ok = loop {
                let num_1 = input[index] - b'0';
                let num_2 = input[index + 1] - b'0';
                let num = (num_1 * 10 + num_2) as usize;

                let ok = rules[num - 11] & seen == 0;
                index += 3;

                if !ok {
                    // loop until eol or eof, skipping the other numbers (they dont matter anymore)
                    while index < input.len() && input[index - 1] != b'\n' {
                        index += 3;
                    }
                    break false;
                }

                // mark as seen
                seen |= 1 << num;

                if index >= input.len() || input[index - 1] == b'\n' {
                    break true;
                }
            };

            if ok {
                // find the middle number by checking the length of the line we processed
                let len = index - start_index;
                let count = len / 3;
                let number_index = count / 2;
                let byte_index = number_index * 3;
                sum += (input[start_index + byte_index] - b'0') as usize * 10;
                sum += (input[start_index + byte_index + 1] - b'0') as usize;
            }
        }

        sum
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let rules: Vec<(usize, usize)> = rules
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('|').unwrap();
                (fast_parse_int(l), fast_parse_int(r))
            })
            .collect();
        let updates: Vec<Vec<usize>> = updates
            .lines()
            .map(|line| line.split(',').map(fast_parse_int).collect())
            .collect();

        updates
            .into_iter()
            .filter(|u| {
                // inverted
                !rules.iter().all(|r| {
                    match (
                        u.iter().position(|&x| x == r.0),
                        u.iter().position(|&x| x == r.1),
                    ) {
                        (Some(p0), Some(p1)) => p0 <= p1,
                        _ => true,
                    }
                })
            })
            .map(|mut u| {
                let mut correct = false;

                while !correct {
                    rules.iter().for_each(|r| {
                        let a = u.iter().position(|&x| x == r.0);
                        let b = u.iter().position(|&x| x == r.1);

                        if let (Some(a), Some(b)) = (a, b) {
                            if a > b {
                                u.swap(a, b);
                            }
                        }
                    });

                    correct = rules.iter().all(|r| {
                        match (
                            u.iter().position(|&x| x == r.0),
                            u.iter().position(|&x| x == r.1),
                        ) {
                            (Some(p0), Some(p1)) => p0 <= p1,
                            _ => true,
                        }
                    });
                }

                u
            })
            .map(|u| u[u.len() / 2])
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(143, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(4185, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(123, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(4480, output);
}
