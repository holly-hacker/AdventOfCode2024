use tinyvec::ArrayVec;
use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 2;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let nums = line
                    .split(' ')
                    .map(fast_parse_int)
                    .collect::<ArrayVec<[_; 8]>>();

                let all_safe_inc = nums
                    .windows(2)
                    .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) <= 3);
                let all_safe_inc_or_dec = all_safe_inc
                    || nums
                        .windows(2)
                        .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) <= 3);

                all_safe_inc_or_dec
            })
            .map(|l| l as usize)
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let nums = line
                    .split(' ')
                    .map(fast_parse_int)
                    .collect::<ArrayVec<[_; 8]>>();

                (0..nums.len()).any(|skip| {
                    let mut nums = nums.clone();
                    nums.remove(skip);

                    let all_safe_inc = nums
                        .windows(2)
                        .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) <= 3);
                    let all_safe_inc_or_dec = all_safe_inc
                        || nums
                            .windows(2)
                            .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) <= 3);

                    all_safe_inc_or_dec
                })
            })
            .map(|l| l as usize)
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(2, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(383, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(4, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(436, output);
}
