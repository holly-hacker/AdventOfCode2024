use regex::Regex;
use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 3;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        // mul\((\d+),(\d+)\)
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        regex
            .captures_iter(input)
            .map(|ca| {
                fast_parse_int(ca.get(1).unwrap().as_str())
                    * fast_parse_int(ca.get(2).unwrap().as_str())
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let mut enable = true;
        let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        regex
            .captures_iter(input)
            .map(|ca| {
                match ca.get(0).unwrap().as_str().as_bytes()[2] {
                    b'l' => {
                        // mul
                        enable as usize
                            * fast_parse_int(ca.get(1).unwrap().as_str())
                            * fast_parse_int(ca.get(2).unwrap().as_str())
                    }
                    b'(' => {
                        // do
                        enable = true;
                        0
                    }
                    b'n' => {
                        // don't
                        enable = false;
                        0
                    }
                    _ => {
                        unreachable!()
                    }
                }
            })
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(161, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(174561379, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(48, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(106921067, output);
}
