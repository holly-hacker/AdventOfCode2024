use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 7;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let (out, rest) = line.split_once(':').unwrap();
                let out = fast_parse_int(out);
                let rest = rest
                    .trim()
                    .split(" ")
                    .map(fast_parse_int)
                    .collect::<Vec<_>>();

                for i in 0..(1 << (rest.len())) {
                    let mut i = i;
                    let mut test_output = 0;
                    for num in &rest {
                        let operation = i & 1;
                        i >>= 1;

                        if operation == 0 {
                            test_output += num;
                        } else {
                            test_output *= num;
                        }
                    }

                    if test_output == out {
                        return out;
                    }
                }
                0
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let (out, rest) = line.split_once(':').unwrap();
                let out = fast_parse_int(out);
                let rest = rest
                    .trim()
                    .split(" ")
                    .map(fast_parse_int)
                    .collect::<Vec<_>>();

                for i in 0..(3usize.pow(rest.len() as u32)) {
                    let mut i = i;
                    let mut test_output = 0;
                    for num in &rest {
                        let operation = i % 3;
                        i /= 3;

                        match operation {
                            1 => test_output += num,
                            2 => test_output *= num,
                            _ => test_output = fast_parse_int(&format!("{test_output}{num}")),
                        }
                    }

                    if test_output == out {
                        return out;
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
