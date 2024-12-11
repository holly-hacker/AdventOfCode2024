use std::collections::HashMap;

use fnv::FnvBuildHasher;
use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut memoize = HashMap::<_, _, FnvBuildHasher>::default();

        input
            .split(' ')
            .map(fast_parse_int)
            .map(|num| resolve_number(num as u64, 25, &mut memoize))
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let mut memoize = HashMap::<_, _, FnvBuildHasher>::default();

        input
            .split(' ')
            .map(fast_parse_int)
            .map(|num| resolve_number(num as u64, 75, &mut memoize))
            .sum()
    }
}

fn resolve_number(
    num: u64,
    depth: u8,
    memoize: &mut HashMap<(u64, u8), usize, FnvBuildHasher>,
) -> usize {
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
    let half_splitoff = 10u64.pow(half);
    let left_half = num / half_splitoff;
    let right_half = num % half_splitoff;

    let res = resolve_number(left_half, depth - 1, memoize)
        + resolve_number(right_half, depth - 1, memoize);
    memoize.insert((num, depth), res);
    res
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
