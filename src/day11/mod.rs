use std::collections::HashMap;

use utils::{fast_parse_int, fast_parse_int_from_bytes};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut nums = input.split(' ').map(fast_parse_int).collect::<Vec<_>>();

        for _ in 0..25 {
            nums = nums
                .into_iter()
                .flat_map(|num| {
                    let digit_count = num.to_string().len();
                    match num {
                        0 => vec![1],
                        _ if digit_count % 2 == 0 => vec![
                            fast_parse_int_from_bytes(
                                &num.to_string().as_bytes()[digit_count / 2..],
                            ),
                            fast_parse_int_from_bytes(
                                &num.to_string().as_bytes()[..digit_count / 2],
                            ),
                        ],
                        _ => vec![num * 2024],
                    }
                })
                .collect();
        }

        nums.len()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let nums = input.split(' ').map(fast_parse_int).collect::<Vec<_>>();

        let mut memoize = HashMap::new();

        nums.into_iter()
            .map(|num| resolve_number(num, 75, &mut memoize))
            .sum()
    }
}

fn resolve_number(num: usize, depth: usize, memoize: &mut HashMap<(usize, usize), usize>) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(val) = memoize.get(&(num, depth)) {
        return *val;
    }

    if num == 0 {
        let res = resolve_number(1, depth - 1, memoize);
        memoize.insert((num, depth), res);
        return res;
    }

    let digit_count = num.ilog10() + 1;
    if digit_count % 2 != 0 {
        let res = resolve_number(num * 2024, depth - 1, memoize);
        memoize.insert((num, depth), res);
        return res;
    }

    let half = digit_count / 2;
    let left_half = num / 10usize.pow(half);
    let right_half = num % 10usize.pow(half);

    let res1 = resolve_number(left_half, depth - 1, memoize);
    let res2 = resolve_number(right_half, depth - 1, memoize);
    memoize.insert((num, depth), res1 + res2);
    res1 + res2
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(55312, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(218956, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(65601038650482, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(259593838049805, output);
}
